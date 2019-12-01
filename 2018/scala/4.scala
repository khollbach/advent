import scala.io.Source

case class Night(
  guard: Int,
  asleep: List[Boolean]
) {
  // todo
  //require(asleep.length == 60)
}

sealed trait Event {
  def minute: Int
}

case class WakeUp(
  minute: Int
) extends Event

case class FallAsleep(
  minute: Int
) extends Event

object Solution {
  def main(args: Array[String]): Unit = {
    val lines = Source.fromFile("../inputs/4").getLines.toSeq

    val nights = parseEvents(lines.sorted)
  }

  def parseEvents(lines: Seq[String]): Seq[Night] = {
    val lineRegex = raw"\[\d{4}-\d{2}-\d{2} \d{2}:(\d{2})\] (.*)\n?".r

    val beginRegex = raw"Guard #(\d+) begins shift".r
    val sleepRegex = "falls asleep".r
    val wakeRegex = "wakes up".r

    case class PartialNight(
      guard: Int,
      events: List[Event]
    )

    var currentOpt: Option[PartialNight] = None

    val partialNights =
      lines.flatMap {
        case lineRegex(minute, event) =>
          event match {
            case beginRegex(id) =>
              val ret = currentOpt
              currentOpt = Some(PartialNight(id.toInt, Nil))
              ret
            case sleepRegex if currentOpt.nonEmpty =>
              currentOpt =
                currentOpt.map { current =>
                  current.copy(
                    events = FallAsleep(minute.toInt) :: current.events)
                }
              None
            case wakeRegex if currentOpt.nonEmpty =>
              currentOpt =
                currentOpt.map { current =>
                  current.copy(
                    events = WakeUp(minute.toInt) :: current.events)
                }
              None
          }
      }

    def buildNight(partial: PartialNight): Night = {
      Night(0, Nil)
    }

    partialNights.map(buildNight)
  }
}
