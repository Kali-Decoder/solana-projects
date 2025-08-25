import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Dicepool } from "../target/types/dicepool";
import { expect } from "chai";
const web3 = anchor.web3;

describe("dicepool", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const user = (provider.wallet as anchor.Wallet).payer;
  console.log("user address :", user.publicKey.toBase58());
  const program = anchor.workspace.Dicepool as Program<Dicepool>;
  const user0 = anchor.web3.Keypair.generate();
  const user1 = anchor.web3.Keypair.generate();
  const user2 = anchor.web3.Keypair.generate();
  const user3 = anchor.web3.Keypair.generate();
  const user4 = anchor.web3.Keypair.generate();
  const user5 = anchor.web3.Keypair.generate();
  const user6 = anchor.web3.Keypair.generate();
  const user7 = anchor.web3.Keypair.generate();

  const poolId = new anchor.BN(1);
  it("Create Pool", async () => {
    // start time in past 
    const startTime = new anchor.BN(Math.floor(Date.now() / 1000) - 3600);
    const endTime = new anchor.BN(Math.floor(Date.now() / 1000) + 7200);
    const capacity = new anchor.BN(10); // example capacity
    const baseAmount = new anchor.BN(50000000); // example base amount

    const { poolPda } = await getPoolPda(program.programId, user, new anchor.BN(poolId));
    console.log("poolPda:", poolPda.toBase58());

    await program.methods
      .createPool(poolId, startTime, endTime, capacity, baseAmount)
      .accounts({
        payer: user.publicKey,
        dicePool: poolPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    const poolAccount = await program.account.dicePool.fetch(poolPda);
    expect(poolAccount.id.toNumber()).to.equal(poolId.toNumber());

  });

  it("User should Bet in Pool", async () => {
    let { poolPda } = await getPoolPda(program.programId, user, new anchor.BN(poolId));

    const users = [user0, user1, user2, user3, user4, user5, user6, user7];

    for (let i = 0; i < users.length; i++) {
      await airdrop(provider.connection, users[i].publicKey, 10000000000);
      console.log(`Airdropped 10 SOL to user${i} : ${users[i].publicKey.toBase58()}`);
      const betAmount = new anchor.BN(50000000);
      let randomNumber = Math.floor(Math.random() * 6) + 1; // Random number between 1 and 6
      let { playerPda } = await getPlayerPda(program.programId, users[i], new anchor.BN(poolId));
      await program.methods
        .joinPool(poolId, betAmount, new anchor.BN(randomNumber))
        .accounts({
          payer: users[i].publicKey,
          dicePool: poolPda,
          dicePlayer: playerPda,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([users[i]])
        .rpc();
      console.log(`User${i} placed a bet in pool ${poolId.toNumber()}`);
    }
    const poolAccount = await program.account.dicePool.fetch(poolPda);
    console.log({ poolAccount })
    expect(poolAccount.betters.length).to.equal(8);
    expect(poolAccount.totalAmount.toNumber()).to.equal(400000000);
    expect(poolAccount.ended).to.equal(false);


  });

  it("User should Bet in Pool", async () => {
    let { poolPda } = await getPoolPda(program.programId, user, new anchor.BN(poolId));
    let poolAccountBefore = await program.account.dicePool.fetch(poolPda);
    let beforeBalance = await provider.connection.getBalance(poolPda);

    await program.methods.withdrawAll(poolId).accounts({
      creator: user.publicKey,
      dicePool: poolPda,
    }).signers([user]).rpc();
    let poolAccountAfter = await program.account.dicePool.fetch(poolPda);
    let afterBalance = await provider.connection.getBalance(poolPda);

    console.log("Before Balance:", beforeBalance);
    console.log("After Balance:", afterBalance);
  });
});


const getPoolPda = async (programID, user, poolId) => {
  const [poolPda, poolBump] = await web3.PublicKey.findProgramAddressSync(
    [Buffer.from("dice_pool"), user.publicKey.toBuffer(), poolId.toArrayLike(Buffer, "le", 8)],
    programID
  );
  return { poolPda, poolBump };
};

const getPlayerPda = async (programID, user, poolId) => {
  const [playerPda, playerBump] = await web3.PublicKey.findProgramAddressSync(
    [Buffer.from("dice_player"), user.publicKey.toBuffer(), poolId.toArrayLike(Buffer, "le", 8)],
    programID
  );
  return { playerPda, playerBump };
};


async function airdrop(connection: any, address: any, amount = 5000000000) {
  await connection.confirmTransaction(await connection.requestAirdrop(address, amount), "confirmed");
}