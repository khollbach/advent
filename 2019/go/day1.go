package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	masses := readStdin()
	fmt.Println(totalFuel(masses, fuelRequired))
	fmt.Println(totalFuel(masses, fuelRequiredRecursive))
}

func readStdin() []int {
	var xs []int
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		x, err := strconv.Atoi(scanner.Text())
		if err != nil {
			panic(err)
		}
		xs = append(xs, x)
	}
	if err := scanner.Err(); err != nil {
		panic(err)
	}
	return xs
}

func fuelRequired(mass int) int {
	return mass/3 - 2
}

func fuelRequiredRecursive(mass int) int {
	f := fuelRequired(mass)
	if f < 0 {
		return 0
	} else {
		return f + fuelRequiredRecursive(f)
	}
}

func totalFuel(masses []int, fuelRequired func(int) int) int {
	total := 0
	for _, mass := range masses {
		total += fuelRequired(mass)
	}
	return total
}
