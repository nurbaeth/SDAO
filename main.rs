use anchor_lang::prelude::*;

#[program]
pub mod thanos_dao {
    use super::*;

    pub fn create_proposal(ctx: Context<CreateProposal>, description: String) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.description = description;
        proposal.vote_count = 0;
        proposal.executed = false;
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        require!(!proposal.executed, CustomError::AlreadyExecuted);
        require!(!ctx.accounts.voter.has_voted, CustomError::AlreadyVoted);
        
        proposal.vote_count += 1;
        ctx.accounts.voter.has_voted = true;
        Ok(())
    }

    pub fn execute_proposal(ctx: Context<ExecuteProposal>) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        require!(proposal.vote_count > 0, CustomError::NotEnoughVotes);
        require!(!proposal.executed, CustomError::AlreadyExecuted);
        
        proposal.executed = true;
        Ok(())
    }
}

#[account]
pub struct Proposal {
    pub description: String,
    pub vote_count: u64,
    pub executed: bool,
}

#[account]
pub struct Voter {
    pub has_voted: bool,
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(init, payer = user, space = 8 + 100)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    #[account(init, payer = user, space = 8 + 1)]
    pub voter: Account<'info, Voter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteProposal<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[error_code]
pub enum CustomError {
    #[msg("Proposal has already been executed.")]
    AlreadyExecuted,
    #[msg("You have already voted.")]
    AlreadyVoted,
    #[msg("Not enough votes to execute the proposal.")]
    NotEnoughVotes,
}
