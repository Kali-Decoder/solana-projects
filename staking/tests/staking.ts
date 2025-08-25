import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Staking } from "../target/types/staking";
import { expect } from "chai";
const web3 = anchor.web3;

describe("staking", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const user = (provider.wallet as anchor.Wallet).payer;
  console.log("user address :", user.publicKey.toBase58());
  const program = anchor.workspace.Staking as Program<Staking>;
  const owner = anchor.web3.Keypair.generate();
  
  before(async () => {
    const balance = await provider.connection.getBalance(user.publicKey);
    const balanceInSOL = balance / web3.LAMPORTS_PER_SOL;
    const formattedBalance = new Intl.NumberFormat().format(balanceInSOL);
    await airdrop(program.provider.connection, owner.publicKey);
    console.log(`Balance: ${formattedBalance} SOL`);
    console.log(`Balance of Owner `, (await provider.connection.getBalance(owner.publicKey))/web3.LAMPORTS_PER_SOL );
  });

  it("Initialize Vault !!!", async () => {
    let vaultPda = await getVaultPda(program.programId);
    console.log("Vault PDA address:", vaultPda.toBase58());
    await program.methods.initializeVault().accounts({
      vault: vaultPda,
      owner: owner.publicKey,
      systemProgram: web3.SystemProgram.programId,
    }).signers([owner]).rpc();

    console.log("Vault initialized successfully!");
  });
  it("Create Your Account !!!", async () => {
    let pda_account = await getUserAccount(program.programId, user);
    console.log("pda_account address :", pda_account.toBase58());
    await program.methods.initialize().accounts({
      user: user.publicKey,
      userAccount: pda_account,
      systemProgram: web3.SystemProgram.programId,
    }).signers([user]).rpc();
    const account = await program.account.stakeAccount.fetch(pda_account);
    expect(account.user.toBase58()).to.equal(user.publicKey.toBase58());
    expect(account.stakedAmount.toNumber()).to.equal(0);
    expect(account.rewardAmount.toNumber()).to.equal(0);
    expect(account.isActive).to.equal(true);
    console.log("Your account has been created !!!");
  });


  it("Stake some SOL !!!", async () => {
    let pda_account = await getUserAccount(program.programId, user);
    let vaultPda = await getVaultPda(program.programId);
    console.log("pda_account address :", pda_account.toBase58());
    const stakeAmount = 1 * web3.LAMPORTS_PER_SOL; // 1 SOL
    await program.methods.stake(new anchor.BN(stakeAmount)).accounts({
      user: user.publicKey,
      userAccount: pda_account,
      vault:vaultPda,
      systemProgram: web3.SystemProgram.programId,
    }).signers([user]).rpc();
    const account = await program.account.stakeAccount.fetch(pda_account);
    expect(account.stakedAmount.toNumber()).to.equal(stakeAmount);
    expect(account.isActive).to.equal(true);
    const balance = await provider.connection.getBalance(vaultPda);
    console.log("You have staked 1 SOL !!!", { balance });
  });

  it("Get Current Points Corresponding to user", async () => {
    let pda_account = await getUserAccount(program.programId, user);
    console.log("pda_account address :", pda_account.toBase58());

    await program.methods.getPoints().accounts({
      owner: user.publicKey,
      userAccount: pda_account
    }).signers([user]).rpc();

    let data = await program.account.stakeAccount.fetch(pda_account);
    console.log("Current Points :", data.rewardAmount.toNumber());
  });

  it("Unstake your money !!!", async () => {
    let pda_account = await getUserAccount(program.programId, user);
    let vaultPda = await getVaultPda(program.programId);
    const balance = await provider.connection.getBalance(pda_account);
    console.log("pda_account address :", pda_account.toBase58(), { balance });
    const unstakeAmount = new anchor.BN(web3.LAMPORTS_PER_SOL / 2);
    await program.methods.unstake(unstakeAmount).accounts({
      user: user.publicKey,
      userAccount: pda_account,
      vault:vaultPda,
      systemProgram: web3.SystemProgram.programId,
    }).signers([user]).rpc();

    const account = await program.account.stakeAccount.fetch(pda_account);
    expect(account.stakedAmount.toNumber()).to.equal(0.5 * web3.LAMPORTS_PER_SOL);
    expect(account.isActive).to.equal(true);
    console.log("You have unstaked 0.5 SOL !!!", { account });

  });
});


const getUserAccount = async (programID, user) => {
  let [pda_account, _] = await web3.PublicKey.findProgramAddressSync(
    [Buffer.from("stake_account"), user.publicKey.toBuffer()],
    programID
  );
  return pda_account;
}

const getVaultPda = async (programID) => {
  const [vaultPda, _] = await web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault")],
    programID
  );
  return vaultPda;
};

async function airdrop(connection: any, address: any, amount = 5000000000) {
  await connection.confirmTransaction(await connection.requestAirdrop(address, amount), "confirmed");
}