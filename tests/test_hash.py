import unittest

from astryx import gaqwh


class TestAstryx(unittest.TestCase):

    def test_determinism(self):
        msg = "test block data for blockchain"
        self.assertEqual(gaqwh(msg), gaqwh(msg))

    def test_avalanche_effect(self):
        # Test sensitivity to single bit change
        h1 = gaqwh("Astryx1")
        h2 = gaqwh("Astryx2")

        bin1 = bin(int(h1, 16))[2:].zfill(256)
        bin2 = bin(int(h2, 16))[2:].zfill(256)
        diff = sum(c1 != c2 for c1, c2 in zip(bin1, bin2))

        # Expect ~50% change (128 bits). 100 is a safe threshold for "chaotic"
        self.assertGreater(diff, 100, f"Weak avalanche: only {diff} bits changed")

    def test_wallet_keys(self):
        """Test with simulated private keys and addresses."""
        key1 = "5Kb8kLf9zgWQandEC27nYPGZizS8469C365Z"  # Mock Base58
        key2 = "5Kb8kLf9zgWQandEC27nYPGZizS8469C365a"  # 1 char diff

        h1 = gaqwh(key1)
        h2 = gaqwh(key2)
        self.assertNotEqual(h1, h2)

        # Hex key test
        hex_key = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        self.assertEqual(len(gaqwh(hex_key)), 64)

    def test_collision_resistance_small_set(self):
        """Ensure no collisions in a small set of words."""
        words = ["blockchain", "crypto", "astryx", "quantum", "mainnet", "validator"]
        hashes = [gaqwh(w) for w in words]
        self.assertEqual(
            len(hashes), len(set(hashes)), "Collision detected in small set"
        )

    def test_large_input(self):
        """Test with a larger input."""
        large_data = "A" * 64 * 1024
        h = gaqwh(large_data)
        self.assertEqual(len(h), 64)


if __name__ == "__main__":
    unittest.main()
