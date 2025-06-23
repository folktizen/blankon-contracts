import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BlankonContracts } from "../target/types/blankon_contracts";

describe("blankon-contracts", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.blankonContracts as Program<BlankonContracts>;
  const blankonState = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize(
        new anchor.web3.PublicKey("2uPQGpm8X4ZkxMHxrAW1QuhXcse1AHEgPih6Xp9NuEWW"),
        new anchor.web3.PublicKey("7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE"),
        new anchor.web3.PublicKey("4cSM2e6rvbGQUFiJbqytoVMi5GgghSMr8LwVrT9VPSPo")
      )
      .accounts({
        blankonState: blankonState.publicKey,
        admin: anchor.AnchorProvider.env().wallet.publicKey,
      })
      .signers([blankonState])
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
