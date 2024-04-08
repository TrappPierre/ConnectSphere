const express = require('express');
const router = express.Router();
const ArtworkController = require('../controllers/ArtworkController');

// Define artwork-related routes
router.get('/artworks', ArtworkController.getAllArtworks);
router.get('/artworks/:id', ArtworkController.getArtworkById);
router.post('/artworks', ArtworkController.createArtwork);
router.put('/artworks/:id', ArtworkController.updateArtwork);
router.delete('/artworks/:id', ArtworkController.deleteArtwork);
router.get('/artworks/:id/auctions', ArtworkController.getArtworkAuctions);

module.exports = router;

