import sys
import argparse
from astryx import gaqwh

def main():
    parser = argparse.ArgumentParser(description="Astryx (GAQWH) Hashing CLI")
    parser.add_argument("data", nargs='?', help="Data to hash (reads from stdin if omitted)")
    parser.add_argument("-b", "--bits", type=int, default=256, help="Output hash bits (default: 256)")
    
    args = parser.parse_args()
    
    if args.data is None:
        if not sys.stdin.isatty():
            data = sys.stdin.read().strip()
        else:
            print("No data provided. Use 'python cli.py hello' or 'echo hello | python cli.py'")
            return
    else:
        data = args.data
        
    result = gaqwh(data, output_bits=args.bits)
    print(result)

if __name__ == "__main__":
    main()
