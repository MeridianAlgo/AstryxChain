from astryx import gaqwh

def produce_demo():
    test_cases = [
        ("Word: 'Astryx'", "Astryx"),
        ("Word: 'astryx'", "astryx"),
        ("Wallet Key (Mock):", "5Kb8kLf9zgWQandEC27nYPGZizS8469C365Z"),
        ("Ethereum Address (Mock):", "0x742d35Cc6634C0532925a3b844Bc454e4438f44e"),
        ("Large Transaction Data:", "tx_in:0x123...tx_out:0x456...value:100BTC"),
    ]

    print("=== Astryx GAQWH Result Production ===")
    print("-" * 50)
    for label, data in test_cases:
        result = gaqwh(data)
        print(f"{label:<25} | {result}")
    print("-" * 50)

if __name__ == "__main__":
    produce_demo()
