#!/bin/bash
gcc "$1" -o outputfile && ./outputfile && rm outputfile
