#! /usr/bin/env python3
import sys
import email.policy
from email import message_from_file

def usage():
    print("usage: checkdefects.py TESTFILE")

def main(args):
    if not len(args) == 2:
        usage()
        sys.exit(1)
    with open(args[1]) as fd:
        mail = message_from_file(fd, policy=email.policy.strict)

    if not len(mail.defects) == 0:
        print(mail.defects, file=sys.stderr)
        sys.exit(1)
    print("No defects\n--", file=sys.stderr)


if __name__ == "__main__":
    main(sys.argv)
