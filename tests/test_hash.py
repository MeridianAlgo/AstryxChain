import unittest
import os
import subprocess
import sys

from astryx import gaqwh


class TestAstryx(unittest.TestCase):

    def test_determinism(self):
        msg = "test block data for blockchain"
        self.assertEqual(gaqwh(msg), gaqwh(msg))

    def test_str_and_bytes_match(self):
        msg = "AstryxChain"
        self.assertEqual(gaqwh(msg), gaqwh(msg.encode("utf-8")))

    def test_output_is_hex_and_correct_length_default(self):
        h = gaqwh("Astryx")
        self.assertEqual(len(h), 64)
        int(h, 16)

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
        words = [
            "blockchain",
            "crypto",
            "astryx",
            "quantum",
            "mainnet",
            "validator",
            "consensus",
            "merkle",
            "transaction",
            "difficulty",
            "node",
            "staking",
        ]
        hashes = [gaqwh(w) for w in words]
        self.assertEqual(
            len(hashes), len(set(hashes)), "Collision detected in small set"
        )

    def test_large_input(self):
        """Test with a larger input."""
        large_data = "A" * 64 * 1024
        h = gaqwh(large_data)
        self.assertEqual(len(h), 64)

    def test_variable_output_bits(self):
        self.assertEqual(len(gaqwh("x", output_bits=128)), 32)
        self.assertEqual(len(gaqwh("x", output_bits=256)), 64)
        self.assertEqual(len(gaqwh("x", output_bits=512)), 128)

    def test_invalid_output_bits(self):
        with self.assertRaises(ValueError):
            gaqwh("x", output_bits=0)
        with self.assertRaises(ValueError):
            gaqwh("x", output_bits=-64)
        with self.assertRaises(ValueError):
            gaqwh("x", output_bits=257)


class TestCLI(unittest.TestCase):
    def _run_cli(self, args, input_text=None):
        root = os.path.dirname(os.path.dirname(__file__))
        cli_path = os.path.join(root, "cli.py")
        cmd = [sys.executable, cli_path] + args
        proc = subprocess.run(
            cmd,
            input=input_text,
            text=True,
            capture_output=True,
            cwd=root,
        )
        return proc

    def test_cli_hash_argument(self):
        proc = self._run_cli(["Astryx"], input_text=None)
        self.assertEqual(proc.returncode, 0, proc.stderr)
        out = proc.stdout.strip()
        self.assertEqual(len(out), 64)
        int(out, 16)

    def test_cli_hash_stdin(self):
        proc = self._run_cli([], input_text="Astryx\n")
        self.assertEqual(proc.returncode, 0, proc.stderr)
        out = proc.stdout.strip()
        self.assertEqual(len(out), 64)
        int(out, 16)

    def test_cli_bits_flag(self):
        proc = self._run_cli(["-b", "128", "Astryx"], input_text=None)
        self.assertEqual(proc.returncode, 0, proc.stderr)
        out = proc.stdout.strip()
        self.assertEqual(len(out), 32)

    def test_cli_no_input_prints_message(self):
        proc = self._run_cli([], input_text=None)
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertIn("No data provided", proc.stdout)


if __name__ == "__main__":
    unittest.main()
