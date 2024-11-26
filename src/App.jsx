import {useState, useEffect} from "react";
import {invoke} from "@tauri-apps/api/core";
import "./App.css";

function App() {
    const [data, setData] = useState([]);

    useEffect(() => {
        async function get_logs() {
            invoke("get_logs").then(data => setData(data));
        }

        get_logs();
    }, []);

    return (
        <>
            {
                data.map(x => <LinePreview {...x}/>)
            }
        </>
    );
}

const LineNumber = ({line_no}) => {
    return (
        <div className="loglineno">
            <p>{line_no}</p>
        </div>
    )
}

const DateTime = ({time}) => {
    return (
        <div className="logdatetime">
            <p>{time}</p>
        </div>
    )
}

const LogLevel = ({val}) => {
    return (
        <div className="loglevel" >
            <p>{val}</p>
        </div>
    )
}

const LogMessage = ({val}) => {
    return (
        <div className="logmsg">
            <p>{val}</p>
        </div>
    )
}

const LinePreview = ({line_no, datetime, log_level, message}) => {
    return (
        <>
            <div className="log">
                <LineNumber line_no={line_no}/>
                <DateTime time={datetime}/>
                <LogLevel val={log_level}/>
                <LogMessage val={message}/>
            </div>
        </>
    )
}

export default App;
