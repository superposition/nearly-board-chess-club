import 'regenerator-runtime/runtime';
import React from 'react';

import './assets/global.css';

import { EducationalText, SignInPrompt, SignOutButton, RoomDetails } from './ui-components';
import { boardChessBoard } from './near-interface';


export default function App({ isSignedIn, bcc, wallet }) {
  const [valueFromBlockchain, setValueFromBlockchain] = React.useState();

  const [uiPleaseWait, setUiPleaseWait] = React.useState(true);

  // Get blockchian state once on component load
  React.useEffect(() => {
    bcc.get_fen()
      .then(setValueFromBlockchain)
      .catch(alert)
      .finally(() => {
        setUiPleaseWait(false);
      });
    
  }, []);

  /// If user not signed-in with wallet - show prompt
  if (!isSignedIn) {
    // Sign-in flow will reload the page later
    return <SignInPrompt greeting={valueFromBlockchain} onClick={() => wallet.signIn()}/>;
  }

  function changeGreeting(e) {
    e.preventDefault();
    setUiPleaseWait(true);
    const { greetingInput } = e.target.elements;
    // bcc.setGreeting(greetingInput.value)
    //   .then(async () => {return helloNEAR.getGreeting();})
    //   .then(setValueFromBlockchain)
    //   .finally(() => {
    //     setUiPleaseWait(false);
    //   });
  }

  return (
    <>
      <br/>
      <SignOutButton accountId={wallet.accountId} onClick={() => wallet.signOut()}/>
      <br/>
      <br/>
      <br/>
      <RoomDetails/>

      <main className={uiPleaseWait ? 'please-wait' : ''}>
        <h1>
          The contract says: <span className="greeting">{valueFromBlockchain}</span>
        </h1>
        <form onSubmit={changeGreeting} className="change">
          <label>Change greeting:</label>
          <div>
            <input
              autoComplete="off"
              defaultValue={valueFromBlockchain}
              id="greetingInput"
            />
            <button>
              <span>Save</span>
              <div className="loader"></div>
            </button>
          </div>
        </form>
      </main>
    </>
  );
}
