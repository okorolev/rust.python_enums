import shlex
from setuptools import setup, find_packages

from setuptools_rust import RustExtension

# setuptools       40.7.1
# setuptools-rust  0.10.6
# wheel            0.32.3

setup_requires = [
    "setuptools-rust>=0.10.1",
    "wheel"
]
install_requires = []

setup(
    name="string_sum",
    version="0.1.0",
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "Programming Language :: Python",
        "Programming Language :: Rust",
        "Operating System :: POSIX",
        "Operating System :: MacOS :: MacOS X",
    ],
    packages=find_packages(),
    rust_extensions=[RustExtension(
        "string_sum",
        rustc_flags=shlex.split('-C link-arg=-undefined -C link-arg=dynamic_lookup')
    )],
    install_requires=install_requires,
    setup_requires=setup_requires,
    include_package_data=True,
    zip_safe=False,
)
