from setuptools import find_packages, setup

setup(
    name="astryx",
    version="1.1.0",
    packages=find_packages(),
    install_requires=[
        "numpy>=1.20.0",
    ],
    author="Astryx",
    description="Astryx: High-Performance Quantum-Resistant Blockchain Hashing",
    long_description=open("README.md").read(),
    long_description_content_type="text/markdown",
    url="https://github.com/MeridianAlgo/AstryxChain",
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
        "Topic :: Security :: Cryptography",
    ],
    python_requires=">=3.7",
)
