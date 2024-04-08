const Artwork = require('../models/Artwork');

// Define methods for artwork-related operations
const ArtworkController = {
  getAllArtworks: async (req, res) => {
    try {
      const artworks = await Artwork.find();
      res.json(artworks);
    } catch (error) {
      res.status(500).json({ error: 'Internal server error' });
    }
  },

  getArtworkAuctions: async (req, res) => {
    const artworkId = req.params.id;
    try {
      // Fetch auctions related to the specified artwork
      const auctions = await Auction.find({ artwork: artworkId });
      res.json(auctions);
    } catch (error) {
      res.status(500).json({ error: 'Internal server error' });
    }
  },

  // Implement other CRUD operations and business logic as needed
};

module.exports = ArtworkController;

