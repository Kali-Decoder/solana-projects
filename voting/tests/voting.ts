import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Voting } from "../target/types/voting";
import { assert } from "chai";
const web3 = anchor.web3;
describe("voting", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const user = (provider.wallet as anchor.Wallet).payer;
  const someRandomGuy = anchor.web3.Keypair.generate();
  const program = anchor.workspace.Voting as Program<Voting>;


  let firstPollDetails = {
    poll_id: 1,
    title: "First Poll",
    description: "This is the first poll",
    start_time: new anchor.BN(Date.now() / 1000),
    end_time: new anchor.BN(Date.now() / 1000 + 86400), // 1 day later
  }
  before(async () => {
    const balance = await provider.connection.getBalance(user.publicKey);
    const balanceInSOL = balance / web3.LAMPORTS_PER_SOL;
    const formattedBalance = new Intl.NumberFormat().format(balanceInSOL);
    console.log(`Balance: ${formattedBalance} SOL`);
  });

  it("Creates a new poll", async () => {

    const [pollAddress, pollBump] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("poll"), new anchor.BN(1).toArrayLike(Buffer, "le", 8)],
      program.programId
    );
    const tx = await program.methods.initializePoll(
      new anchor.BN(1),
      new anchor.BN(0),
      new anchor.BN(1759508293),
      "test-poll",
      "description",
    ).rpc();

    // right assert statements 
    const pollAccount = await program.account.pollAccount.fetch(pollAddress);
    assert.ok(pollAccount.pollName === "test-poll");
    assert.ok(pollAccount.pollDescription === "description");
    console.log("Poll created successfully!");
    console.log('Your transaction signature', tx);
  });


  it("Inititalize Candidates", async () => {
    const pollIdBuffer = new anchor.BN(1).toArrayLike(Buffer, "le", 8);
    const [pollAddress] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("poll"), pollIdBuffer],
      program.programId
    );

    const nikku = await program.methods.initializeCandidate(
      new anchor.BN(1),
      "Neeraj Choubisa").accounts({
        pollAccount: pollAddress,
      }).rpc();

    const sneha = await program.methods.initializeCandidate(
      new anchor.BN(1),
      "Sneha").accounts({
        pollAccount: pollAddress,
      }).rpc();
    console.log('Your transaction signature', nikku, sneha);

  });

  it("Vote for a candidate", async () => {
    const tx = await program.methods.vote(
      new anchor.BN(1),
      "Neeraj Choubisa").rpc();
      console.log("Vote cast successfully!");
      console.log('Your transaction signature', tx);
  });

  it("Fetch Candidate Details corresponding to Id ", async () => {
    const pollIdBuffer = new anchor.BN(1).toArrayLike(Buffer, "le", 8);
    const [candidatePDAddress] = web3.PublicKey.findProgramAddressSync(
      [pollIdBuffer,Buffer.from("Neeraj Choubisa")],
      program.programId
    );

    const account = await program.account.candidateAccount.fetch(candidatePDAddress);

    console.log(account);

  });

  it("Fetch Poll Details", async () => {
    const pollIdBuffer = new anchor.BN(1).toArrayLike(Buffer, "le", 8);
    const [pollAddress] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("poll"), pollIdBuffer],
      program.programId
    );

    const account = await program.account.pollAccount.fetch(pollAddress);

    console.log(account);
  }
  );
});
