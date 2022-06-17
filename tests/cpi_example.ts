import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Keypair } from '@solana/web3.js'; //Redundant as anchor already includes web3.js lib
import { expect } from 'chai';
import { Puppet } from '../target/types/puppet';
import { PuppetMaster } from '../target/types/puppet_master';

describe('puppet', () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const puppetProgram = anchor.workspace.Puppet as Program<Puppet>;
  const puppetMasterProgram = anchor.workspace.PuppetMaster as Program<PuppetMaster>;

  //const puppetKeypair = Keypair.generate();

  it('Account initialised and count has been incremented!', async () => {

    //Generate an account to emulate a user account
    const kp = anchor.web3.Keypair.generate();

    console.log("User Public key: ", kp.publicKey);

    // await function returns the tx signature (to use with block explorer)
    const tx = await puppetProgram.methods.initialize().accounts({
      puppet: kp.publicKey, // New puppet account will be owned by the signer ('kp' signer creates new account)
      authority: puppetMasterProgram.programId, // Authority for interacting with functions will be puppetMaster Pk
    }).signers([kp]).rpc();

    // Where do I get "remoteIncrement" from? Answer: "../target/types/puppet"
    const tx2 = await puppetMasterProgram.methods.remoteIncrement(new anchor.BN(10)).accounts({
      puppet: kp.publicKey, // User (kp) owns the public key which was used in the initialise function
      //puppet_program: puppetMasterProgram.programId, // Why doesn't this work???
      puppetProgram: puppetProgram.programId, // Address of the puppet program (same as signer 'kp')
      //signer: puppetMasterProgram.programId, // Signer public key (Must bePuppet wallet authority)
      authority: puppetMasterProgram.programId, // Signer public key (Must bePuppet wallet authority)

    }).signers([kp]).rpc();

    
    // console.log("Puppet Public key: ", puppetProgram.programId);
    // console.log("Master Puppet Public key: ", puppetMasterProgram.programId);

    // expect((await puppetProgram.account.data
    //   .fetch(puppetKeypair.publicKey)).data.toNumber()).to.equal(42);
  });
});