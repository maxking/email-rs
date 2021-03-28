#! /usr/bin/env python3
import sys
from email.policy import default as default_policy
from email import message_from_file

def usage():
    print("usage: checkdefects.py TESTFILE")

def main(args):
    if not len(args) == 2:
        usage()
        sys.exit(1)
    with open(args[1]) as fd:
        mail = message_from_file(fd, policy=default_policy)
    if not len(mail.defects) == 0:
        print(mail.defects, file=sys.stderr)
        sys.exit(1)
    # Check if both python and library serializes to same thing.
    with open(args[1]) as fd:
        mail_str = fd.read()
    if mail_str != mail.as_string(policy=default_policy):
        print('Serialized forms are not same', file=sys.stderr)
        print(f'Input: \n{mail_str}\n---', file=sys.stderr)
        print('Expected: \n{}'.format(mail.as_string(policy=default_policy)),
              file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main(sys.argv)
