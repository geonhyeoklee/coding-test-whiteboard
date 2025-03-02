function solution(video_len, pos, op_start, op_end, commands) {
  const _video_len = get_seconds(video_len);
  const _op_start = get_seconds(op_start);
  const _op_end = get_seconds(op_end);

  let _pos = get_seconds(pos);

  if (check_in_boundary(_pos, _op_start, _op_end)) {
    _pos = _op_end;
  }

  commands.forEach((command) => {
    command === "next" ? (_pos += 10) : (_pos -= 10);
    _pos = clamp_pos(_pos, 0, _video_len);

    if (check_in_boundary(_pos, _op_start, _op_end)) {
      _pos = _op_end;
    }
  });

  const minutes = String(Math.floor(_pos / 60)).padStart(2, "0");
  const seconds = String(_pos % 60).padStart(2, "0");

  return `${minutes}:${seconds}`;
}

function check_in_boundary(pos, start, end) {
  return start <= pos && pos <= end;
}

function clamp_pos(pos, min, max) {
  return Math.max(min, Math.min(pos, max));
}

function get_seconds(time) {
  const _time = time.split(":");
  const seconds = Number(_time[0]) * 60 + Number(_time[1]);
  return seconds;
}

const video_len = "30:35";
const pos = "30:30";
const op_start = "01:00";
const op_end = "02:00";
const commands = ["next"];

console.log(solution(video_len, pos, op_start, op_end, commands));
