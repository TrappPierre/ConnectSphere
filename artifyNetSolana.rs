use ink_lang as ink;

#[ink::contract]
mod nft_auction {
    use ink_prelude::vec::Vec;
    use ink_storage::{
        collections::{HashMap as StorageHashMap, Vec as StorageVec},
        lazy::Lazy,
    };
    
    #[ink(storage)]
    pub struct NftAuction {
        owner: AccountId,
        nfts: StorageHashMap<TokenId, Nft>,
        auctions: StorageHashMap<TokenId, Auction>,
        tokens: StorageVec<TokenId>,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        token_id: TokenId,
    }

    #[ink(event)]
    pub struct AuctionStarted {
        #[ink(topic)]
        token_id: TokenId,
        #[ink(topic)]
        seller: AccountId,
        reserve_price: Balance,
        duration: BlockNumber,
    }

    #[ink(event)]
    pub struct AuctionEnded {
        #[ink(topic)]
        token_id: TokenId,
        #[ink(topic)]
        winner: Option<AccountId>,
        winning_bid: Balance,
    }

    #[ink(storage)]
    pub struct Nft {
        owner: AccountId,
        metadata_uri: Lazy<Hash>,
    }

    #[ink(storage)]
    pub struct Auction {
        seller: AccountId,
        highest_bidder: Option<AccountId>,
        highest_bid: Balance,
        reserve_price: Balance,
        end_block: BlockNumber,
    }

    #[ink(event)]
    pub struct BidPlaced {
        #[ink(topic)]
        token_id: TokenId,
        bidder: AccountId,
        amount: Balance,
    }

    impl NftAuction {
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            Self {
                owner: caller,
                nfts: Default::default(),
                auctions: Default::default(),
                tokens: Default::default(),
            }
        }

        #[ink(message)]
        pub fn mint(&mut self, metadata_uri: Hash) -> TokenId {
            let caller = self.env().caller();
            assert_eq!(self.owner, caller, "Only owner can mint NFTs");

            let token_id = self.tokens.len() as TokenId;
            self.tokens.push(token_id);
            self.nfts.insert(token_id, Nft {
                owner: caller,
                metadata_uri: Lazy::new(metadata_uri),
            });

            self.env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                token_id,
            });

            token_id
        }

        #[ink(message)]
        pub fn start_auction(&mut self, token_id: TokenId, reserve_price: Balance, duration: BlockNumber) {
            let caller = self.env().caller();
            let nft = self.nfts.get_mut(&token_id).unwrap();
            assert_eq!(nft.owner, caller, "Only NFT owner can start an auction");

            self.auctions.insert(token_id, Auction {
                seller: caller,
                highest_bidder: None,
                highest_bid: 0,
                reserve_price,
                end_block: self.env().block_number() + duration,
            });

            self.env().emit_event(AuctionStarted {
                token_id,
                seller: caller,
                reserve_price,
                duration,
            });
        }

        #[ink(message)]
        pub fn place_bid(&mut self, token_id: TokenId, amount: Balance) {
            let caller = self.env().caller();
            let auction = self.auctions.get_mut(&token_id).unwrap();

            assert!(self.env().block_number() < auction.end_block, "Auction has ended");
            assert!(amount >= auction.reserve_price, "Bid must be at least the reserve price");
            assert!(amount > auction.highest_bid, "Bid must be higher than the current highest bid");

            if let Some(previous_bidder) = auction.highest_bidder {
                // Refund the previous highest bidder
                self.env().transfer(previous_bidder, auction.highest_bid).unwrap();
            }

            auction.highest_bidder = Some(caller);
            auction.highest_bid = amount;

            self.env().emit_event(BidPlaced {
                token_id,
                bidder: caller,
                amount,
            });
        }

        #[ink(message)]
        pub fn end_auction(&mut self, token_id: TokenId) {
            let caller = self.env().caller();
            let auction = self.auctions.get(&token_id).unwrap();
            assert!(self.env().block_number() >= auction.end_block, "Auction has not ended yet");
            assert_eq!(auction.seller, caller, "Only the seller can end the auction");

            if let Some(highest_bidder) = auction.highest_bidder {
                // Transfer NFT ownership to the highest bidder
                let mut nft = self.nfts.get_mut(&token_id).unwrap();
                nft.owner = highest_bidder;

                self.env().emit_event(Transfer {
                    from: Some(caller),
                    to: Some(highest_bidder),
                    token_id,
                });

                self.env().emit_event(AuctionEnded {
                    token_id,
                    winner: Some(highest_bidder),
                    winning_bid: auction.highest_bid,
                });
            } else {
                // No bids received, return the NFT to the seller
                self.env().emit_event(Transfer {
                    from: Some(caller),
                    to: Some(auction.seller),
                    token_id,
                });

                self.env().emit_event(AuctionEnded {
                    token_id,
                    winner: None,
                    winning_bid: 0,
                });
            }

            self.auctions.remove(&token_id);
        }
    }
}
