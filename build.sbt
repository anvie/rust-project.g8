scalaVersion := "2.10.7"

seq(giter8Settings :_*)

resolvers ++= Seq(
  "Sonatype Releases" at "https://oss.sonatype.org/content/groups/scala-tools",
  "typesafe repo"   at "http://repo.typesafe.com/typesafe/releases/"
)
