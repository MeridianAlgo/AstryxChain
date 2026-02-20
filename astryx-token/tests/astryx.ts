import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Astryx } from "../target/types/astryx";
import { 
  TOKEN_2022_PROGRAM_ID, 
  getAssociatedTokenAddressSync, 
  createAssociatedTokenAccountInstruction 
} from "@solana/spl-token";
import { expect } from "chai";

describe("astryx", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Astryx as Program<Astryx>;
  const authority = provider.wallet.publicKey;

  it("Is initialized!", async () => {
    const total_supply = new anchor.BN(1000000000).mul(new anchor.BN(10**9));
    const team_vest_end = new anchor.BN(Math.floor(Date.now() / 1000) + 3600); // 1 hour from now
    
    const [configPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("config"), authority.toBuffer()],
      program.programId
    );

    const [stakingPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("staking_pool"), authority.toBuffer()],
      program.programId
    );

    const [treasuryPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("treasury"), authority.toBuffer()],
      program.programId
    );

    const mint = anchor.web3.Keypair.generate();

    const tx = await program.methods.initialize({
      totalSupply: total_supply,
      teamVestEnd: team_vest_end,
      multiOracleConsensus: true,
      multisigSigners: [authority, authority, authority, authority, authority] // Dummy for test
    })
    .accounts({
      config: configPda,
      stakingPool: stakingPda,
      treasury: treasuryPda,
      mint: mint.publicKey,
      authority: authority,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
    })
    .signers([mint])
    .rpc();

    console.log("Your transaction signature", tx);
    
    const config = await program.account.config.fetch(configPda);
    expect(config.totalSupply.toString()).to.equal(total_supply.toString());
  });

  // More tests for stake, vote, transfer_hook block, etc.
});
