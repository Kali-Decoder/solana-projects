import * as anchor from '@coral-xyz/anchor';
import type { Program } from '@coral-xyz/anchor';
import { getCustomErrorMessage } from '@solana-developers/helpers';
import { assert } from 'chai';
import type { Favorites } from '../target/types/favorites';
import { systemProgramErrors } from './system-errors';
const web3 = anchor.web3;

describe("favorites", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const user = (provider.wallet as anchor.Wallet).payer;
  const someRandomGuy = anchor.web3.Keypair.generate();
  const program = anchor.workspace.Favorites as Program<Favorites>;


  const favoriteNumber = new anchor.BN(42);
  const favoriteColor = 'blue';
  const favoriteHOBBIES = ['SKIING', 'READING', 'TRAVELING'];


  before(async () => {
    const balance = await provider.connection.getBalance(user.publicKey);
    const balanceInSOL = balance / web3.LAMPORTS_PER_SOL;
    const formattedBalance = new Intl.NumberFormat().format(balanceInSOL);
    console.log(`Balance: ${formattedBalance} SOL`);
  });


  it('Writes our favorites to the blockchain', async () => {
    await program.methods.setFavorites(favoriteNumber, favoriteColor, favoriteHOBBIES).signers([user]).rpc();
    console.log('Your favorites have been written to the blockchain!');


    const favoritesPdsAndBump = web3.PublicKey.findProgramAddressSync(
      [Buffer.from('favorites'), user.publicKey.toBuffer()], program.programId
    );

    const favoritesPda = favoritesPdsAndBump[0];
    const datFromPda = await program.account.favorites.fetch(favoritesPda);

    assert.equal(datFromPda.number.toString(), favoriteNumber.toString(), 'Favorite number does not match');
    assert.equal(datFromPda.color, favoriteColor, 'Favorite color does not match');
    assert.deepEqual(datFromPda.hobbies, favoriteHOBBIES, 'Favorite hobbies do not match');
    console.log('Favorites successfully written and verified!');
  });

  it('Updates the favorites', async () => {
    const newFavoriteHobbies = ['skiing', 'skydiving', 'biking', 'swimming'];
    try {
      await program.methods.setFavorites(favoriteNumber, favoriteColor, newFavoriteHobbies).signers([user]).rpc();
    } catch (error) {
      console.error((error as Error).message);
      const customErrorMessage = getCustomErrorMessage(systemProgramErrors, error);
      throw new Error(customErrorMessage);
    }
  });
  it('Rejects transactions from unauthorized signers', async () => {
    try {
      await program.methods
        // set_favourites in Rust becomes setFavorites in TypeScript
        .setFavorites(favoriteNumber, favoriteColor, favoriteHOBBIES)
        // Sign the transaction
        .signers([someRandomGuy])
        // Send the transaction to the cluster or RPC
        .rpc();
    } catch (error) {
      const errorMessage = (error as Error).message;
      assert.isTrue(errorMessage.includes('unknown signer'));
    }
  });


});
