#!/usr/bin/env bash
yes y | pip uninstall string-sum

echo " ";
echo "RM build"; rm -rf build/
echo "RM dist"; rm -rf dist/
echo "RM egg info"; rm -rf *.egg-*
echo "RM __pycache__"; rm -rf __pycache__
echo " ";


python setup.py install
#>out.log 2>&1

echo "   ";


python -c "from string_sum import PyEnum; print( PyEnum('Kek', {'black': 1}))"
