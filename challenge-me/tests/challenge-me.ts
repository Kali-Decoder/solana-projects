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
  const someRandomGuy = anchor.web3.Keypair.generate();
  const program = anchor.workspace.ChallengeMe as Program<ChallengeMe>;

  const challengeOption = {
    oneWeek: { oneWeek: {} },
    oneMonth: { oneMonth: {} },
    twoMonths: { twoMonths: {} },
    sixMonths: { sixMonths: {} },
    oneYear: { oneYear: {} },
    seventyFiveHard: { seventyFiveHard: {} }
  };

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

  it("Get All Challanges Asociated to user", async () => {
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
        userAccount: userPDA, // user profile does NOT exist
        systemProgram: anchor.web3.SystemProgram.programId,
      }).signers([someRandomGuy]).rpc();
      expect.fail("Expected error for non-existent user profile");
    } catch (err) {
      // Check error type
      console.log("Error caught as expected:", err.error ? err.error.errorMessage : err.toString());
      expect(err.toString()).to.include("AccountNotInitialized"); // Anchor default error for missing account
    }
  });
  it("Upload Updation to challenege | Upload Completed Task on Challenge Tracker", async () => {
    const id = 1;
    const challengePDA = await getChallengePDA(user, program.programId, id);
    const day = 1;
    const postId =1;
    const taskPDA = await getPostPDA(challengePDA, program.programId, postId);

    console.log("challengePDA: ", challengePDA.toBase58());
    console.log("taskPDA: ", taskPDA.toBase58());

    await program.methods.uploadPost(
      "Task One Completed",
      "Task One Completed Done",
      "✅",
      "Today",
      new anchor.BN(day),
      new anchor.BN(postId),
    ).accounts({
      owner: user.publicKey,
      challenge: challengePDA,
      task: taskPDA,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([user]).rpc({skipPreflight:true});

    const afterPostPda = await program.account.task.fetch(taskPDA);
    console.log({afterPostPda}, "afterChallengePda");
    expect(afterPostPda.discription).to.equal("Task One Completed Done");
    expect(afterPostPda.title).to.equal("Task One Completed");
    expect(afterPostPda.day.toNumber()).to.equal(1);

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