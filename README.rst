.. image:: https://img.shields.io/badge/License-Apache%202.0-blue.svg
    :target: https://opensource.org/licenses/Apache-2.0

pwrgen
======

*Password and passphrase generator*

CLI tool to generate either a passphrase or a password.

Examples
--------

Passphrase with two words:

.. code:: shell

    $ pwrgen phrase -w2
    mucorrhea synonymy

Passphrase with three words, and override the word separator:

.. code:: shell

    $ pwrgen phrase -w3 -s "_"
    haab_scribatious_phacoidoscope

Password with default settings (default length is 16):

.. code:: shell

    $ pwrgen password
    T-_e:XQj7P{;>KqE

Password with custom settings. Disable numbers (``-n0``), use a custom
set of special characters (``-s'!@#$'``), and also disable uppercase
letters:

.. code:: shell

    $ ./target/release/pwrgen password -n0 -s'!@#$' -u0
    $mbyqdxu!okj$hnk

Passphrase
----------

.. code:: shell

    $ pwrgen phrase -h
    Passphrase made up of dictionary words

    USAGE:
        pwrgen phrase [OPTIONS]

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
        -s, --separator <separator>      Word separator for passphrase generation [default:  ]
        -w, --word_count <word_count>     [default: 4]

Password
--------

.. code:: shell

    $ pwrgen password -h
    Password made up of symbols

    USAGE:
        pwrgen password [OPTIONS]

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
        -l, --length <length>      Password length [default: 16]
        -w, --lower <lower>        Include lowercase ASCII letters [default: 1]
        -n, --numbers <numbers>    Include numbers [default: 1]
        -s, --special <special>    Include special characters [default: !@#$%^&*(){}[]-+=_:;<>?]
        -u, --upper <upper>        Include uppercase ASCII letters [default: 1]

