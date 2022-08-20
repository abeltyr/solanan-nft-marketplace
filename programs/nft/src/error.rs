//! Module provide program defined errors

use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    // 5000
    #[msg("Nft Is Already Listed")]
    NftAlreadyListed,
    // 5001
    #[msg("Provided Token Account don't match signer")]
    InvalidTokenAccount,
    // 5002
    #[msg("Listing is not activated or setup")]
    ListingNotActivate,
    // 5003
    #[msg("Listing doesn't have the price setup")]
    ListingPriceNotSet,
    // 5004
    #[msg("Listing has been closed")]
    ListingAlreadyClosed,
    // 5005
    #[msg("Nft is not Listed")]
    NftNotListed,
    // 5006
    #[msg("Seller Token Account has not been delegated")]
    TokenAccountNotDelegated,
    // 5007
    #[msg("Seller Token Account not delegated to the program")]
    InvalidTokenAccountDelegation,
    // 5008
    #[msg("Seller Token Account is not owner of the nft")]
    TokenAccountOwnerIssue,
    // 5009
    #[msg("Issue with the data provided")]
    DataIssue,
    // 5010
    #[msg("StartDate cannot be in the past")]
    StartDateIsInPast,
    // 5011
    #[msg("EndDate should not be earlier than StartDate")]
    EndDateIsEarlierThanBeginDate,
    // 5012
    #[msg("Invalid Data input given")]
    InvalidData,
    // 5013
    #[msg("Auction not started yet")]
    AuctionNotStarted,
    // 5014
    #[msg("Auction has ended")]
    AuctionEnded,
    // 5015
    #[msg("Auction Bid lower than Highest bidder")]
    BidLowerThanHighestBider,
    // 5016
    #[msg("Auction Bid lower than startling bidder")]
    BidLowerThanStartingBid,
    // 5017
    #[msg("Auction has not been set yet")]
    AuctionNotSet,
    // 5017
    #[msg("The seller can't be bid")]
    SellerBidIssue,
}
