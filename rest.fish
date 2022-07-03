set base http://localhost:3000

function do_curl -a method path headers data
  echo $data | curl -i --data @- --request $method -H $headers "$base$path"
end

function r-notify-mail
  set headers "Content-Type: application/json"
  set data '{
    "subject": "test note",
    "body": "message body"
  }'

  do_curl POST /notify/mail $headers $data
end

function r-notify-text
  set headers "Content-Type: application/json"
  set data '{
    "subject": "test note",
    "body": "message body"
  }'

  do_curl POST /notify/text $headers $data
end
