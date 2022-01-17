import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Client } from '../target/types/client';

describe('client', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Client as Program<Client>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
