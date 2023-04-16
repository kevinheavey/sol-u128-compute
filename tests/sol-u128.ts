import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolU128 } from "../target/types/sol_u128";

describe("sol-u128", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolU128 as Program<SolU128>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
