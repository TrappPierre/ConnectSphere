const express = require('express');
const router = express.Router();
const AuctionController = require('../controllers/AuctionController');

// Define auction-related routes
router.get('/auctions', AuctionController.getAllAuctions);
router.get('/auctions/:id', AuctionController.getAuctionById);
router.post('/auctions', AuctionController.createAuction);
router.put('/auctions/:id', AuctionController.updateAuction);
router.delete('/auctions/:id', AuctionController.deleteAuction);
router.get('/auctions/:id/bids', AuctionController.getAuctionBids);

module.exports = router;
