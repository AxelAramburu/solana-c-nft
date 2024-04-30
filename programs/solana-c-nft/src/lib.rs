use account_compression_cpi::Noop;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        mint_to, 
        Mint, MintTo, Token, TokenAccount
    },
    metadata::{
        create_master_edition_v3, 
        create_metadata_accounts_v3, 
        sign_metadata, 
        Metadata, SignMetadata, CreateMasterEditionV3, CreateMetadataAccountsV3, MetadataAccount,

    },
};
use mpl_token_metadata::{
    accounts::{MasterEdition, Metadata as MetadataMPL},
    types::{CollectionDetails, DataV2, Creator as CreatorMPL},
};
use bubblegum_cpi::{
    program::Bubblegum,
    cpi::{
        accounts::{CreateTree, MintToCollectionV1},
        create_tree, 
        mint_to_collection_v1,
    },
    Collection, Creator, MetadataArgs, TokenProgramVersion, TokenStandard,
};
use account_compression_cpi::{
    program::SplAccountCompression
};

pub const SEED: &str = "RSTSeed";
declare_id!("3raZxd2zrYgu3qQ8mwpKBK3wxoUBedHm6Lt5GPpMzSaB");

#[program]
pub mod solana_c_nft {

    use super::*;   

    pub fn init_cnft(ctx: Context<InitCNFT>, name: String, symbol: String, uri: String, ) -> Result<()> {
        // PDA signer
        let pda_signer_seed: &[&[&[u8]]] = &[&[SEED.as_bytes(), &[ctx.bumps.collection_mint]]];

        // create mint account
        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.collection_mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.collection_mint.to_account_info(),
            },
            &pda_signer_seed
        );

        mint_to(cpi_context, 1)?;   

        // create metadata account
        let cpi_context_metadata = CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                mint: ctx.accounts.collection_mint.to_account_info(),
                mint_authority: ctx.accounts.collection_mint.to_account_info(),
                payer: ctx.accounts.signer.to_account_info(),
                update_authority: ctx.accounts.signer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            &pda_signer_seed
        );

        let data_v2 = DataV2 {
            name: name,
            symbol: symbol,
            uri: uri,
            seller_fee_basis_points: 0,
            creators: Some(vec![CreatorMPL {
                address: ctx.accounts.signer.key(),
                verified: false,
                share: 100,
            }]),
            collection: None,
            uses: None,
        };

        create_metadata_accounts_v3(cpi_context_metadata, data_v2, true, true, Some(CollectionDetails::V1 { size: 0 }),)?;

        //create master edition account
        let cpi_context_master_edition = CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMasterEditionV3 {
                edition: ctx.accounts.master_edition_account.to_account_info(),
                mint: ctx.accounts.collection_mint.to_account_info(),
                update_authority: ctx.accounts.signer.to_account_info(),
                mint_authority: ctx.accounts.collection_mint.to_account_info(),
                payer: ctx.accounts.signer.to_account_info(),
                metadata: ctx.accounts.metadata_account.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            &pda_signer_seed
        );
        create_master_edition_v3(cpi_context_master_edition, Some(0))?;

        sign_metadata(CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            SignMetadata {
                creator: ctx.accounts.signer.to_account_info(),
                metadata: ctx.accounts.metadata_account.to_account_info(),
            },
        ))?;
        
        Ok(())
    }

    pub fn init_tree(ctx: Context<InitTree>) -> Result<()> {

        let pda_signer_seed: &[&[&[u8]]] = &[&[SEED.as_bytes(), &[ctx.bumps.collection_mint]]];

        let cpi_context_tree = CpiContext::new_with_signer(
            ctx.accounts.bubblegum_program.to_account_info(),
            CreateTree {
                tree_authority: ctx.accounts.tree_authority.to_account_info(),
                merkle_tree: ctx.accounts.merkle_tree.to_account_info(),
                payer: ctx.accounts.signer.to_account_info(),
                tree_creator: ctx.accounts.collection_mint.to_account_info(), // set creator as pda
                log_wrapper: ctx.accounts.log_wrapper.to_account_info(),
                compression_program: ctx.accounts.compression_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
            &pda_signer_seed
        );

        // create the tree
        create_tree(cpi_context_tree, 14, 64, Option::from(false))?;

        Ok(())
    }

    pub fn mint_cnft(ctx: Context<MintCNft>) -> Result<()> {

        let pda_signer_seed: &[&[&[u8]]] = &[&[SEED.as_bytes(), &[ctx.bumps.collection_mint]]];
        
        // use collection nft metadata as the metadata for the compressed nft
        let metadata_account = &ctx.accounts.collection_metadata;

        let metadata = MetadataArgs {
            name: metadata_account.name.to_string(),
            symbol: metadata_account.symbol.to_string(),
            uri: metadata_account.uri.to_string(),
            collection: Some(Collection {
                key: ctx.accounts.collection_mint.key(),
                verified: false,
            }),
            primary_sale_happened: true,
            is_mutable: true,
            edition_nonce: None,
            token_standard: Some(TokenStandard::NonFungible),
            uses: None,
            token_program_version: TokenProgramVersion::Original,
            creators: vec![Creator {
                address: ctx.accounts.collection_mint.key(),
                verified: true,
                share: 100,
            }],
            seller_fee_basis_points: 200, // e.g. 2% 
        };

        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.bubblegum_program.to_account_info(),
            MintToCollectionV1 {
                tree_authority: ctx.accounts.tree_authority.to_account_info(),
                leaf_owner: ctx.accounts.signer.to_account_info(),
                leaf_delegate: ctx.accounts.signer.to_account_info(),
                merkle_tree: ctx.accounts.merkle_tree.to_account_info(),
                payer: ctx.accounts.signer.to_account_info(),
                tree_delegate: ctx.accounts.collection_mint.to_account_info(), // tree delegate is pda, required as a signer
                collection_authority: ctx.accounts.signer.to_account_info(), // collection authority is pda (nft metadata update authority)
                collection_authority_record_pda: ctx.accounts.bubblegum_program.to_account_info(),
                collection_mint: ctx.accounts.collection_mint.to_account_info(), // collection nft mint account
                collection_metadata: ctx.accounts.collection_metadata.to_account_info(), // collection nft metadata account
                edition_account: ctx.accounts.edition_account.to_account_info(), // collection nft master edition account
                bubblegum_signer: ctx.accounts.bubblegum_signer.to_account_info(),
                log_wrapper: ctx.accounts.log_wrapper.to_account_info(),
                compression_program: ctx.accounts.compression_program.to_account_info(),
                token_metadata_program: ctx.accounts.token_metadata_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
            &pda_signer_seed
        );
        mint_to_collection_v1(cpi_context, metadata)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitCNFT<'info> {
    #[account(mut, signer)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [SEED.as_bytes()],
        bump,
        payer = signer,
        mint::decimals = 0,
        mint::authority = collection_mint,
        mint::freeze_authority = collection_mint
    )]
    pub collection_mint: Account<'info, Mint>,
    
    /// CHECK: address
    #[account(
        mut,
        address=MetadataMPL::find_pda(&collection_mint.key()).0
    )]
    pub metadata_account: UncheckedAccount<'info>,
    
    /// CHECK: address
    #[account(
        mut,
        address=MasterEdition::find_pda(&collection_mint.key()).0
    )]
    pub master_edition_account: UncheckedAccount<'info>,
    
    /// CHECK:
    #[account(
        mut,
        seeds = [merkle_tree.key().as_ref()],
        bump,
        seeds::program = bubblegum_program.key()
    )]
    pub tree_authority: UncheckedAccount<'info>,
    
    /// CHECK:
    #[account(mut)]
    pub merkle_tree: UncheckedAccount<'info>,
    

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = collection_mint,
        associated_token::authority = signer
    )]
    pub token_account: Account<'info, TokenAccount>, 

    pub token_program: Program<'info, Token>, 
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub bubblegum_program: Program<'info, Bubblegum>,
    pub compression_program: Program<'info, SplAccountCompression>,
}

#[derive(Accounts)]
pub struct InitTree<'info> {
    #[account(mut, signer)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED.as_bytes()],
        bump,
        mint::decimals = 0,
    )]
    pub collection_mint: Account<'info, Mint>,

    /// CHECK:
    #[account(
            mut,
            seeds = [merkle_tree.key().as_ref()],
            bump,
            seeds::program = bubblegum_program.key()
        )]
    pub tree_authority: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub merkle_tree: UncheckedAccount<'info>,
    pub log_wrapper: Program<'info, Noop>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub bubblegum_program: Program<'info, Bubblegum>,
    pub compression_program: Program<'info, SplAccountCompression>,
}

#[derive(Accounts)]
pub struct MintCNft<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK:
    #[account(
        seeds = [SEED.as_bytes()],
        bump,
    )]
    pub collection_mint: UncheckedAccount<'info>,

    /// CHECK:
    #[account(
        mut,
        seeds = [merkle_tree.key().as_ref()],
        bump,
        seeds::program = bubblegum_program.key()
    )]
    pub tree_authority: UncheckedAccount<'info>,

    /// CHECK:
    #[account(mut)]
    pub merkle_tree: UncheckedAccount<'info>,

    /// CHECK:
    #[account(
        seeds = ["RSTSeed".as_bytes()],
        seeds::program = bubblegum_program.key(),
        bump,
    )]
    pub bubblegum_signer: UncheckedAccount<'info>,

    pub log_wrapper: Program<'info, Noop>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    
    #[account(mut)]
    pub collection_metadata: Account<'info, MetadataAccount>,
    /// CHECK:
    pub edition_account: UncheckedAccount<'info>,
    pub bubblegum_program: Program<'info, Bubblegum>,
    pub compression_program: Program<'info, SplAccountCompression>,
}