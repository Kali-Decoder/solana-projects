import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

import { Bank } from "../target/types/bank"; 
import { expect } from 'chai'; 
const BANK_ACCOUNT_SEED = "bank";

describe("bank", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Bank as Program<Bank>;
  const bankcreator = anchor.web3.Keypair.generate();
  const bankName = "Nikku";
  it("Create Banke",async ()=>{
    await airdrop(program.provider.connection, bankcreator.publicKey);

    const [bank_publickey, bank_bump]  = await getBankAddress(bankcreator.publicKey, program.programId);

    await program.methods.create(bankName).accounts({
      owner: bankcreator.publicKey,
      bankAccount: bank_publickey,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([bankcreator]).rpc();

    let bankAccountData = await program.account.bankAccount.fetch(bank_publickey);
    expect(bankAccountData.name).to.eql(bankName)
    expect(bankAccountData.owner).to.eql(bankcreator.publicKey)

  });

  it("Deposit into Bank",async ()=>{
    const [bank_publickey, bank_bump] = getBankAddress(bankcreator.publicKey, program.programId);
    let depositAmount = new anchor.BN(500);
    await program.methods.deposit(depositAmount).accounts({
      owner: bankcreator.publicKey,
      bankAccount: bank_publickey,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([bankcreator]).rpc();

    let bankAccountData = await program.account.bankAccount.fetch(bank_publickey);
    expect(bankAccountData.balance.toNumber()).to.eql(depositAmount.toNumber());
  });


  it("Withdraw from Bank ",async () =>{
    const [bank_publickey, bank_bump] = getBankAddress(bankcreator.publicKey, program.programId);
    let amount = new anchor.BN(300);
    let bankAccountDataBefore = await program.account.bankAccount.fetch(bank_publickey);
    await program.methods.withdraw(amount).accounts({
      owner: bankcreator.publicKey,
      bankAccount: bank_publickey,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([bankcreator]).rpc();

    let bankAccountDataAfter = await program.account.bankAccount.fetch(bank_publickey);
    expect(bankAccountDataBefore.balance.toNumber() - amount.toNumber()).to.eql(bankAccountDataAfter.balance.toNumber())

  });


});

function getBankAddress(bankCreator: anchor.web3.PublicKey, programID: anchor.web3.PublicKey) {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode(BANK_ACCOUNT_SEED),
      bankCreator.toBuffer()
    ], programID);
}

async function airdrop(connection: any, address: any, amount = 1000000000) {
  await connection.confirmTransaction(await connection.requestAirdrop(address, amount), "confirmed");
}