#!/usr/bin/scala
!#

import scala.io.Source

object Solution {
  def main(args: Array[String]): Unit = {
    val lines = Source.fromFile("input").getLines

    val ids = lines.toSeq
    println(part1(ids))
    println(part2(ids))
  }

  def part1(ids: Seq[String]): Int = {
    def hasLetterWithExactFrequency(s: String, freq: Int): Boolean = {
      s.exists { c =>
        s.count(_ == c) == freq
      }
    }

    ids.count(hasLetterWithExactFrequency(_, 2)) *
      ids.count(hasLetterWithExactFrequency(_, 3))
  }

  /**
   * Find two ids differing in exactly one coordinate.
   * Return the common subsequence.
   */
  def part2(ids: Seq[String]): String = {
    findMatch(ids).get
  }

  /**
   * Find two ids that differ in exactly one coordinate.
   * Return their common subsequence. None if no such pair is found.
   */
  def findMatch(ids: Seq[String]): Option[String] = {
    val pairs = for (id1 <- ids; id2 <- ids) yield (id1, id2)

    // TODO: how to make pairs not include (x, x) and (y, x) for every (x, y)?
    // TODO: how to make pairs lazy?
    //val pairs = ???

    // This works, but doesn't stop early.
    //pairs.flatMap {
      //case (id1, id2) => isMatch(id1, id2)
    //}.headOption

    // Early-stopping version of the above.
    pairs.collectFirst {
      Function.unlift {
        case (id1, id2) => isMatch(id1, id2)
      }
    }
  }

  /**
   * Do the two ids differ in exactly one coordinate?
   *
   * If so, return either id with that coordinate sliced out. Else, None.
   */
  def isMatch(id1: String, id2: String): Option[String] = {
    if (id1.length != id2.length) None
    else {
      val len = id1.length
      val matchingChars =
        id1.zip(id2)
          .filter { case (c1, c2) => c1 == c2 }
          .map { _._1 }
          .mkString
      if (matchingChars.length == len - 1) {
        Some(matchingChars)
      } else {
        None
      }
    }
  }
}
