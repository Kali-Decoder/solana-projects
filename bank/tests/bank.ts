import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Bank } from "../target/types/bank";
import { assert } from 'chai';
const web3 = anchor.web3;
describe("bank", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const user = (provider.wallet as anchor.Wallet).payer;
  const someRandomGuy = anchor.web3.Keypair.generate();
  const program = anchor.workspace.Bank as Program<Bank>;

  before(async () => {
    const balance = await provider.connection.getBalance(user.publicKey);
    const balanceInSOL = balance / web3.LAMPORTS_PER_SOL;
    const formattedBalance = new Intl.NumberFormat().format(balanceInSOL);
    console.log(`Balance: ${formattedBalance} SOL`);
  });

  it("Create bank", async () => {
  

    const bankAccountSeed = "bank";
    const [bankAccountPDA] = await anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from(bankAccountSeed),
        user.publicKey.toBuffer()
      ],
      program.programId
    );
    const tx = await program.methods.create("Nikku").accounts({
      owner: user.publicKey,
      bankAccount: bankAccountPDA,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([user]).rpc();


    const account = await program.account.bankAccount.fetch(bankAccountPDA);

    console.log("Account name:", account.name);
    assert.equal(account.name.toString(), "Nikku");
    assert.equal(account.owner.toString(), user.publicKey.toString());
    assert.equal(account.balance,0);
  });
});
