import "./App.css";
import React, { useState, useEffect } from "react";
const riotKey = "api_key=RGAPI-dc84f04c-47a4-4366-969e-76c3b8457736";
const fetch = require("node-fetch");
Matches(fetchSumByName("KumiGaoo", "accountID"));
async function Matches(
  accId,
  champId,
  queue,
  endTime,
  beginTime,
  endIndex,
  beginIndex
) {
  let link =
    "https://euw1.api.riotgames.com/lol/match/v4/matchlists/by-account/" +
    (await accId) +
    "?";
  if (champId != null) link += "champion_" + champId + "&";
  if (queue != null) link += "queue_" + queue + "&";
  if (endTime != null) link += "endTime_" + endTime + "&";
  if (beginTime != null) link += "beginTime_" + beginTime + "&";
  if (endIndex != null) link += "endIndex_" + endIndex + "&";
  if (beginIndex != null) link += "beginIndex_" + beginIndex + "&";
  link += riotKey;

  let response = await fetch(link);
  response = await response.json();
  console.log(response.matches[0].champion);
}

async function fetchSumByName(name, ch) {
  while (name.includes(" ")) {
    let spaceSpot = name.indexOf(" ");
    name = name.substring(0, spaceSpot) + name.substring(spaceSpot + 1);
  }
  const link =
    "https://euw1.api.riotgames.com/lol/summoner/v4/summoners/by-name/KumiGaoo?riotKey";
  const response = await fetch(link);
  let data = await response.json();
  if (ch === "id") {
    console.log("wrong one");
    return data.id;
  } else if (ch === "accountId") return data.accountId;
}
function App() {
  const url =
    "https://euw1.api.riotgames.com/lol/summoner/v4/summoners/by-name/KumiGaoo?$(riotKey)";
  const [data, setData] = useState([]);

  const fetchInfo = () => {
    return fetch(url)
      .then((res) => res.json())
      .then((d) => setData(d));
  };

  useEffect(() => {
    fetchInfo();
  }, []);

  return (
    <div className="Matches">
      <h1 style={{ color: "green" }}>using JavaScript inbuilt FETCH API</h1>
      <center>
        {data.map((dataObj, index) => {
          return (
            <div
              style={{
                width: "15em",
                backgroundColor: "#35D841",
                padding: 2,
                borderRadius: 10,
                marginBlock: 10,
              }}
            >
              <p style={{ fontSize: 20, color: "white" }}>
                {dataObj.summonerLevel}
              </p>
            </div>
          );
        })}
      </center>
    </div>
  );
}

export default App;
