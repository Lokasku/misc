import argparse
import numpy as np
import matplotlib.pyplot as plt

from src.julia import *


if __name__ == "__main__":
    
    julia = argparse.ArgumentParser(prog="jfg", description="Julia fractal generator.") 

    julia.add_argument("pixeldensity", type=int, help="quality. Would be around 1000")
    julia.add_argument("detaildensity", type=int, help="finesse. Would be around 100")
    julia.add_argument("c", type=float, nargs=2, help="complex number 'c' being the starting point of the fractal")

    julia.add_argument("-c", "--colors", type=str, help="to choose the color")
    julia.add_argument("-s", "--save", type=str, help="to save figure")

    args = julia.parse_args()

    
    print(args)
    
    c = complex(args.c[0], args.c[1])

    plan = create_complex_matrix(-2, 2, -2, 2, args.pixeldensity)
    julia = new(plan, c, args.detaildensity)

    if args.colors:
        plt.imshow(julia, cmap=args.colors)
        
        if args.save:
            plt.savefig(args.save) 

        plt.show()       

    else:
        plt.imshow(julia, cmap="gray")

        if args.save:
            plt.savefig(args.save) 

        plt.show()       


