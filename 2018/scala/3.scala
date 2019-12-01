import scala.io.Source

case class Point(
  x: Int,
  y: Int,
)

case class Claim(
  id: Int,
  topLeft: Point,
  width: Int,
  height: Int
) {
  def bottomRight: Point =
    Point(topLeft.x + width, topLeft.y + height)
}

object Solution {
  def main(args: Array[String]): Unit = {
    val lines = Source.fromFile("../inputs/3").getLines
    val claims = parseClaims(lines.toSeq)

    println(part1(claims))
    println(part2(claims))
  }

  def parseClaims(lines: Seq[String]): Seq[Claim] = {
    val regex = raw"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)".r

    lines.map {
      case regex(id, x, y, width, height) =>
        Claim(id.toInt, Point(x.toInt, y.toInt), width.toInt, height.toInt)
    }
  }

  def part1(claims: Seq[Claim]): Int = {
    assert(claims.forall(_.topLeft.x >= 0))
    assert(claims.forall(_.topLeft.y >= 0))

    val claimsPerSquare = numClaimsPerSquare(claims)

    // todo: is there a way to flatten claimsPerSquare first?
    // E.g.: claimsPerSquare.flatten.count(_ >= 2)
    claimsPerSquare.map { row =>
      row.count(_ >= 2)
    }.sum
  }

  def numClaimsPerSquare(claims: Seq[Claim]): Array[Array[Int]] = {
    val xMax = claims.map(_.bottomRight.x).max
    val yMax = claims.map(_.bottomRight.y).max

    val claimsPerSquare = Array.fill(xMax + 1, yMax + 1)(0)

    claims.foreach { claim =>
      for (x <- claim.topLeft.x to claim.bottomRight.x;
           y <- claim.topLeft.y to claim.bottomRight.y)
      yield claimsPerSquare(x)(y) += 1
    }

    claimsPerSquare
  }

  def part2(claims: Seq[Claim]): Int = {
    val claimsPerSquare = numClaimsPerSquare(claims)

    val firstSuccessfulClaim =
      claims.collectFirst {
        Function.unlift { claim =>
          if (isValid(claim, claimsPerSquare)) Some(claim)
          else None
        }
      }.get

    firstSuccessfulClaim.id
  }

  def isValid(claim: Claim, claimsPerSquare: Array[Array[Int]]): Boolean = {
    {
      for (x <- claim.topLeft.x to claim.bottomRight.x;
           y <- claim.topLeft.y to claim.bottomRight.y)
      yield claimsPerSquare(x)(y) == 1
    }.fold(true)(_ && _)
  }
}
