#!/usr/bin/scala
!#

import scala.io.Source

case class Claim(
  id: Int,
  x: Int,
  y: Int,
  width: Int,
  height: Int
) {
  def rightEdge: Int = x + width
  def bottomEdge: Int = y + height
}

object Solution {
  def main(args: Array[String]): Unit = {
    val lines = Source.fromFile("input").getLines

    val claims = parseClaims(lines.toSeq)

    println(part1(claims))

    println(part2(claims))
  }

  def parseClaims(lines: Seq[String]): Seq[Claim] = {
    val regex = {
      val g = raw"(\d+)"
      raw"#$g @ $g,$g: ${g}x$g".r
    }

    lines.map {
      case regex(id, x, y, width, height) =>
        Claim(id.toInt, x.toInt, y.toInt, width.toInt, height.toInt)
    }
  }

  def part1(claims: Seq[Claim]): Int = {
    assert(claims.forall(_.x >= 0))
    assert(claims.forall(_.y >= 0))

    val xMax = claims.map(_.rightEdge).max
    val yMax = claims.map(_.bottomEdge).max

    val freqs = Array.fill(xMax, yMax)(0)

    claims.foreach { claim =>
      for (x <- claim.x until claim.rightEdge;
        y <- claim.y until claim.bottomEdge) {
          freqs(x)(y) += 1
      }
    }

    freqs.map { row =>
      row.count(_ >= 2)
    }.sum
  }

  def part2(claims: Seq[Claim]): Nothing = {
    // todo
    ???
  }
}
