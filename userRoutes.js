const express = require('express');
const router = express.Router();
const UserController = require('../controllers/UserController');

// Define user-related routes
router.get('/users', UserController.getAllUsers);
router.get('/users/:id', UserController.getUserById);
router.post('/users', UserController.createUser);
router.put('/users/:id', UserController.updateUser);
router.delete('/users/:id', UserController.deleteUser);
router.get('/users/:id/artworks', UserController.getUserArtworks);

module.exports = router;
