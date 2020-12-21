package nl.thedutchmc.authserver.session;

import java.sql.ResultSet;
import java.sql.SQLException;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import org.apache.commons.lang3.exception.ExceptionUtils;

import nl.thedutchmc.authserver.App;
import nl.thedutchmc.authserver.User;
import nl.thedutchmc.authserver.database.ResultObject;
import nl.thedutchmc.authserver.database.SqlManager;
import nl.thedutchmc.authserver.database.StatementType;

public class SessionManager {

	public boolean sessionValid(String sessionId) {		
		User user = null;
		for(Map.Entry<String, User> entry : App.userMap.entrySet()) {
			User u = entry.getValue();
			if(u.getSessionId() != null && u.getSessionId().equals(sessionId)) user = u;
		}
		
		if(user != null) return true;
		
		final String statement = "SELECT id FROM users WHERE session='" + sessionId + "'";
				
		SqlManager sqlManager = App.getSqlManager();
		try {
			ResultObject ro = sqlManager.executeStatement(StatementType.query, statement);
			ResultSet rs = ro.getResultSet();
						
			while(rs.next()) {
				String id = rs.getString("id");
								
				user = App.userMap.get(id);
				if(user == null) {
					return false;
				}
				
				return true;
			}
		} catch (SQLException e) {
			App.logError("Something went wrong trying to verify a session, caused by a SQLException.");
			App.logDebug(ExceptionUtils.getStackTrace(e));
		}
		
		return false;
	}
	
	public void createSession(String id, String token, String sessionId) {
		final SqlManager sqlManager = App.getSqlManager();
		
		App.userMap.get(id).setSessionId(sessionId);
		
		try {
			String statement = "SELECT id,session FROM users";
			ResultObject ro = sqlManager.executeStatement(StatementType.query, statement);
			ResultSet rs = ro.getResultSet();
			
			List<String> ids = new ArrayList<>();
			while(rs.next()) {
				ids.add(rs.getString("id"));
			}

			if(ids.contains(id)) {
				statement = "UPDATE users SET token ='" + token +"', session ='" + sessionId + "' WHERE id ='" + id + "'";
				sqlManager.executeStatement(StatementType.update, statement);
			} else {
				statement = "INSERT INTO users (id, token, session) VALUES ('" + id + "','" + token + "','" + sessionId +"')";
				sqlManager.executeStatement(StatementType.update, statement);
			}
			
			
		} catch (SQLException e) {
			App.logError("Something went wrong trying to create a new session, caused by a SQLException");
			App.logDebug(ExceptionUtils.getStackTrace(e));
		}
	}
}
