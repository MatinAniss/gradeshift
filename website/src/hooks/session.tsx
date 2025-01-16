import { refresh } from "@/api/rest/auth";
import { createContext, ReactElement, useContext, useEffect, useState } from "react";

interface Context {
  session: Session;
  setToken: (token: Token | null) => void;
  setUser: (user: User | null) => void;
  refreshToken: () => Promise<void>;
  logout: () => void;
}

interface Session {
  token: Token | null;
  user: User | null;
}

interface Token {
  token: string;
  createdAt: Date;
  expiresAt: Date;
}

interface User {
  id: string;
  email: string;
  firstName: string;
  lastName: string;
}

export let auth: Context = { session: { token: null, user: null }, setToken: () => {}, setUser: () => {}, refreshToken: async () => {}, logout: () => {} };
const AuthContext = createContext<Context>(auth);

export function AuthProvider({ children }: { children: ReactElement }) {
  const [sessionState, setSessionState] = useState<Session>(() => {
    const session = getSessionLocalStorage();
    if (session) {
      return session;
    } else {
      return { token: null, user: null };
    }
  });

  const setToken = (token: Token | null) => {
    setSessionState((oldSession) => {
      const updatedSession: Session = { ...oldSession, token };
      setSessionLocalStorage(updatedSession);
      return updatedSession;
    });
  };

  const setUser = (user: User | null) => {
    setSessionState((oldSession) => {
      const updatedSession: Session = { ...oldSession, user };
      setSessionLocalStorage(updatedSession);
      return updatedSession;
    });
  };

  const refreshToken = async () => {
    if (sessionState.token) {
      const { token } = await refresh();
      setSessionState((oldSession) => {
        const updatedSession: Session = { ...oldSession, token: { token: token.token, createdAt: new Date(token.createdAt), expiresAt: new Date(token.expiresAt) } };
        setSessionLocalStorage(updatedSession);
        return updatedSession;
      });
    }
  };

  const logout = () => {
    const updatedSession: Session = { token: null, user: null };
    setSessionLocalStorage(updatedSession);
    setSessionState(updatedSession);
  };

  auth = { session: sessionState, setToken, setUser, refreshToken, logout };

  useEffect(() => {
    if (auth.session.token) {
      const renewal_date = new Date(auth.session.token.createdAt);
      renewal_date.setDate(auth.session.token.createdAt.getDate() + 3);

      if (auth.session.token.expiresAt < new Date()) {
        auth.setToken(null);
        auth.setUser(null);
      } else if (renewal_date < new Date()) {
        (async () => {
          const { token } = await refresh();
          setSessionState((oldSession) => {
            const updatedSession: Session = { ...oldSession, token: { token: token.token, createdAt: new Date(token.createdAt), expiresAt: new Date(token.expiresAt) } };
            setSessionLocalStorage(updatedSession);
            return updatedSession;
          });
        })();
      }
    }
  }, []);

  return <AuthContext.Provider value={auth}>{children}</AuthContext.Provider>;
}

export function useSession() {
  return useContext(AuthContext);
}

function setSessionLocalStorage(session: Session) {
  localStorage.setItem("session", JSON.stringify(session));
}

function getSessionLocalStorage(): Session | null {
  const auth = localStorage.getItem("session");
  return auth
    ? JSON.parse(auth, (key, value) => {
        if (key === "expiresAt" || key === "createdAt") {
          return new Date(value);
        }
        return value;
      })
    : null;
}
