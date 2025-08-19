import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ChallengeMe } from "../target/types/challenge_me";
import { expect } from "chai";
const web3 = anchor.web3;
describe("challenge-me", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const user = (provider.wallet as anchor.Wallet).payer;
  console.log("user address :", user.publicKey.toBase58());
  
  const program = anchor.workspace.ChallengeMe as Program<ChallengeMe>;
  before(async () => {
    const balance = await provider.connection.getBalance(user.publicKey);
    const balanceInSOL = balance / web3.LAMPORTS_PER_SOL;
    const formattedBalance = new Intl.NumberFormat().format(balanceInSOL);
    console.log(`Balance: ${formattedBalance} SOL`);
  });


  it("Create User Account on Challenge Tracker", async () => {
    let userPDA = await getUserPDA(user, program.programId);
    await program.methods.initialize().accounts({
      userProfile: userPDA,
      user: user.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([user]).rpc();
    const userProfileAfter = await program.account.userProfile.fetch(userPDA);
    expect(userProfileAfter.owner.toBase58()).to.equal(user.publicKey.toBase58());
  });

  it("Take one challenge on Challenge Tracker", async () => {
    let id = 1;
    let challengePDA = await getChallengePDA(user, program.programId, id);
    let userPDA = await getUserPDA(user, program.programId);
    await program.methods.startChallenge(
      new anchor.BN(id),
      { oneWeek: {} }
    ).accounts({
      owner: user.publicKey,
      challenge: challengePDA,
      userAccount: userPDA,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([user]).rpc();
    const afterChallengePda = await program.account.challenge.fetch(challengePDA);
    console.log(afterChallengePda, "afterChallengePda");
    expect(afterChallengePda.totalDays).to.equal(7);
  });

  it("Get All Challanges Asocisated to user", async () => {
    let userPDA = await getUserPDA(user, program.programId);
    const userProfileAfter = await program.account.userProfile.fetch(userPDA);
    for(var i=0; i<userProfileAfter.challenges.length;i++){
      let challangeData = await program.account.challenge.fetch(userProfileAfter.challenges[0]);
      console.log({challangeData},"challangeData");
    }
    expect(userProfileAfter.challenges.length).to.eq(1);
  });

  it("Cannot start challenge if user does not exist", async () => {
    const id = 1;
    const someRandomGuy = anchor.web3.Keypair.generate();
    const challengePDA = await getChallengePDA(someRandomGuy, program.programId, id);
    const userPDA = await getUserPDA(someRandomGuy, program.programId);
    try {
      await program.methods.startChallenge(
        new anchor.BN(id),
        { oneWeek: {} }
      ).accounts({
        owner: someRandomGuy.publicKey,
        challenge: challengePDA,
        userAccount: userPDA, 
        systemProgram: anchor.web3.SystemProgram.programId,
      }).signers([someRandomGuy]).rpc();
      expect.fail("Expected error for non-existent user profile");
    } catch (err) {
      console.log("Error caught as expected:", err.error ? err.error.errorMessage : err.toString());
      expect(err.toString()).to.include("AccountNotInitialized");
    }
  });
  it("Upload multiple posts to challenge | Upload Completed Tasks on Challenge Tracker", async () => {
    const id = 1;
    const challengePDA = await getChallengePDA(user, program.programId, id);
    const posts = [
      { postId: 1, title: "Task One Completed", desc: "Task One Completed Done", icon: "✅", date: "Day 1",day:1 },
      { postId: 2, title: "Task Two Completed", desc: "Task Two Completed Done", icon: "🔥", date: "Day 2",day:2  },
      { postId: 3, title: "Task Three Completed", desc: "Task Three Completed Done", icon: "🚀", date: "Day 3",day:3  },
      { postId: 4, title: "Task Four Completed", desc: "Task Four Completed Done", icon: "🎯", date: "Day 4",day:4  },
      { postId: 5, title: "Task Five Completed", desc: "Task Five Completed Done", icon: "🏆", date: "Day 5",day:5  },
    ];
  
    for (const post of posts) {
      const taskPDA = await getPostPDA(challengePDA, program.programId, post.postId);
  
      await program.methods.uploadPost(
        new anchor.BN(id),
        new anchor.BN(post.postId),
        post.title,
        post.desc,
        post.icon,
        post.date,
        new anchor.BN(post.day),
      ).accounts({
        owner: user.publicKey,
        challenge: challengePDA,
        task: taskPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).signers([user]).rpc();
  
      const afterPostPda = await program.account.task.fetch(taskPDA);
      // Assertions
      expect(afterPostPda.title).to.equal(post.title);
      expect(afterPostPda.discription).to.equal(post.desc);
      expect(afterPostPda.day.toNumber()).to.equal(post.day);
    }
  });

  it("Fetch all posts under one challenge", async () => {
    const id = 1;
    const challengePDA = await getChallengePDA(user, program.programId, id);
  
    let posts: any[] = [];
    for (let postId = 1; postId <= 5; postId++) {
      const taskPDA = await getPostPDA(challengePDA, program.programId, postId);
      try {
        const postAcc = await program.account.task.fetch(taskPDA);
        let _post = {
          postId: postAcc.postId.toNumber(), // BN → number
          owner: postAcc.owner.toBase58(),   // PublicKey → string
          title: postAcc.title,
          description: postAcc.discription,  // typo preserved from struct
          emoji: postAcc.emoji,
          currentTime: postAcc.currentTime,
          challenge: postAcc.challenge.toBase58(), // PublicKey → string
          day: postAcc.day.toNumber()
        }
        posts.push(_post);
      } catch (e) {
        console.log(`Post ${postId} not found`, e.message);
      }
    }
  
    console.log("All posts for challenge", id, posts);
    expect(posts.length).to.equal(5);
    expect(posts[0].title).to.equal("Task One Completed"); // etc.
  });
  
  

});

const getUserPDA = async (user, programID) => {
  const [userPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("user_profile"),
      user.publicKey.toBuffer()
    ],
    programID
  );

  return userPDA;
};

const getChallengePDA = async (user, programID, id) => {
  const [challengePDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("challenge"),
      user.publicKey.toBuffer(),
      new anchor.BN(id).toArrayLike(Buffer, "le", 8),
    ],
    programID
  );

  return challengePDA;
};

const getPostPDA = async (challengePDA, programID,id) => {
  const [taskPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("post"),
      challengePDA.toBuffer(),
      new anchor.BN(id).toArrayLike(Buffer, "le", 8)
    ],
    programID
  );

  return taskPDA;
};