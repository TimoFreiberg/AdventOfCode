def get-input [year, day] {
  (curl $"https://adventofcode.com/($year)/day/($day)/input"
    -X "GET"
    -H $"Cookie: session=(open session.cookie)"
    -H 'Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8'
    -H 'Sec-Fetch-Site: none'
    -H 'Sec-Fetch-Mode: navigate'
    -H 'Host: adventofcode.com'
    -H 'Accept-Language: en-US,en;q=0.9'
    -H 'Sec-Fetch-Dest: document'
    -H 'Connection: keep-alive'
    )
}

def main [year start_day = 1 end_day = 25] {
  for $day in $start_day..$end_day {
    let $input_path = $"($year)/inputs/day($day)"
    if ($input_path | path exists) {
      print $"($input_path) already exists"
      continue
    }
    get-input $year $day | save $input_path
  }
}
