import scala.io.Source
import scala.collection.immutable.HashSet

object Solution {
  def main(args: Array[String]): Unit = {
    val lines = Source.fromFile("../inputs/1").getLines
    val nums = lines.map(_.toInt).toSeq

    println(part1(nums))
    println(part2(nums))
  }

  def part1(nums: Seq[Int]): Int = {
    nums.sum
  }

  def part2(nums: Seq[Int]): Int = {
    require(nums.nonEmpty)

    def firstRepeatedSum(
      currTotal: Int,
      nums: Seq[Int],
      seen: Set[Int]
    ): Int = {
      assert(nums.nonEmpty)
      val newTotal = currTotal + nums.head
      if (seen.contains(newTotal)) {
        newTotal
      } else {
        firstRepeatedSum(newTotal, nums.tail, seen + newTotal)
      }
    }

    lazy val repeatedNums: Stream[Int] = nums.toStream #::: repeatedNums
    firstRepeatedSum(0, repeatedNums, HashSet(0))
  }
}
