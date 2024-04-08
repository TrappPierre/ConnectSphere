const User = require('../models/User');

// Define methods for user-related operations
const UserController = {
  getAllUsers: async (req, res) => {
    try {
      const users = await User.find();
      res.json(users);
    } catch (error) {
      res.status(500).json({ error: 'Internal server error' });
    }
  },

  getUserArtworks: async (req, res) => {
    const userId = req.params.id;
    try {
      // Fetch artworks created by the specified user
      const artworks = await Artwork.find({ creator: userId });
      res.json(artworks);
    } catch (error) {
      res.status(500).json({ error: 'Internal server error' });
    }
  },

  // Implement other CRUD operations and business logic as needed
};

module.exports = UserController;
