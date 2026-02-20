from setuptools import setup, find_packages

setup(
    name="astryx",
    version="0.1.0",
    packages=find_packages(),
    install_requires=[
        "numpy>=1.20.0",
    ],
    author="Grok (Implemented by Gemini)",
    description="Astryx: A Quantum-Inspired Adaptive Walk Hashing Algorithm",
    long_description=open("README.md").read(),
    long_description_content_type="text/markdown",
    url="https://github.com/yourusername/Astryx",
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
        "Topic :: Security :: Cryptography",
    ],
    python_requires='>=3.7',
)
