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
    #[msg("Listing has not started yet")]
    ListingNotStarted,
    // 5014
    #[msg("Listing has been Ended")]
    ListingEnded,
    // 5015
    #[msg("Auction Bid lower than Highest bidder")]
    BidLowerThanHighestBider,
    // 5016
    #[msg("Auction Bid lower than startling bidder")]
    BidLowerThanStartingBid,
    // 5017
    #[msg("Auction has not been set yet")]
    AuctionNotSet,
    // 5018
    #[msg("The seller can't be bid")]
    SellerBidIssue,
    // 5019
    #[msg("The listing is not closed")]
    ListingNotClosed,
    // 5020
    #[msg("The listing is still active")]
    ActiveListing,
    // 5021
    #[msg("The account doesn't have lamports to withdraw")]
    NOLamports,
    // 5021
    #[msg("The highest bidder can't withdraw fund")]
    HighestBidderWithDrawIssue,
    // 5022
    #[msg("The bid provided is an invalid one")]
    BidAccountIssue,
    // 5023
    #[msg("The Withdrawer doesn't have access")]
    UnAuthorizedWithdrawal,
    // 5024
    #[msg("The Given Token is not the owner of the nft")]
    MintTokenIssue,
    // 5025
    #[msg("Only the Seller can close the a listing ")]
    ClosingIssue,
    // 5026
    #[msg("The provided nft listing isn't valid")]
    NftListingInvalidData,
    // 5027
    #[msg("The provided seller isn't valid")]
    SellerInvalidData,
    // 5028
    #[msg("The provided bidder isn't valid")]
    BidderInvalidData,
    // 5029
    #[msg("The starting price must be higher than zero")]
    PriceIssue,
    // 5030
    #[msg("Double withdrawal attempted")]
    DoubleWithdrawIssue,
    // 5031
    #[msg("The provided nft authority isn't valid")]
    NftAuthorityInvalidData,
    // 5032
    #[msg("The seller can't be buy the nft")]
    SellerBuyingIssue,
    // 5033
    #[msg("Nft not transferred seller can't withdraw ")]
    SellerWithdrawIssue,
    // 5034
    #[msg("Auction didn't have any Bids")]
    NoBids,
}
