import unittest
from astryx import gaqwh

class TestAstryx(unittest.TestCase):
    
    def test_determinism(self):
        """Verify that the same input produces the same hash."""
        msg = "test block data for blockchain"
        h1 = gaqwh(msg)
        h2 = gaqwh(msg)
        self.assertEqual(h1, h2, "Hash must be deterministic")

    def test_avalanche_effect(self):
        """Verify that a single bit change in input leads to a vastly different hash."""
        msg1 = "block12345678"
        msg2 = "block12345679"  # 1 bit difference
        h1 = gaqwh(msg1)
        h2 = gaqwh(msg2)
        self.assertNotEqual(h1, h2, "Avalanche effect must be present")
        
        # Check Hamming distance (approximate)
        # Convert hex to bin and compare
        bin1 = bin(int(h1, 16))[2:].zfill(256)
        bin2 = bin(int(h2, 16))[2:].zfill(256)
        diff = sum(c1 != c2 for c1, c2 in zip(bin1, bin2))
        self.assertGreater(diff, 64, "Avalanche effect is weak: less than 25% bits changed")

    def test_empty_input(self):
        """Verify empty input doesn't crash."""
        h = gaqwh("")
        self.assertTrue(len(h) > 0)

    def test_different_lengths(self):
        """Verify different lengths produce unique hashes."""
        h1 = gaqwh("a")
        h2 = gaqwh("aa")
        self.assertNotEqual(h1, h2)

if __name__ == "__main__":
    unittest.main()
