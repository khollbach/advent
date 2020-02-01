object Main {
  def main(args: Array[String]): Unit = {
    val masses: List[Int] = readStdin()
    println(totalFuel(masses, fuelRequired))
    println(totalFuel(masses, fuelRequiredRecursive))
  }

  def readStdin(): List[Int] = {
    io.Source.stdin.getLines().map(_.toInt).toList
  }

  def fuelRequired(mass: Int): Int = mass / 3 - 2

  def fuelRequiredRecursive(mass: Int): Int = {
    val f = fuelRequired(mass)
    if (f < 0) {
      0
    } else {
      f + fuelRequiredRecursive(f)
    }
  }

  def totalFuel(masses: List[Int], fuelRequired: Int => Int): Int =
    masses.map(fuelRequired).sum
}
