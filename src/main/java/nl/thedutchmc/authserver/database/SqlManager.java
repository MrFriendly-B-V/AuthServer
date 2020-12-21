package nl.thedutchmc.authserver.database;

import java.sql.Connection;
import java.sql.DriverManager;
import java.sql.PreparedStatement;
import java.sql.ResultSet;
import java.sql.SQLException;

import org.apache.commons.lang3.exception.ExceptionUtils;

import nl.thedutchmc.authserver.App;
import nl.thedutchmc.authserver.Config;

public class SqlManager {

	private Connection connection;
	
	public SqlManager() {
		App.logInfo("Initializing database connector...");
		
		try {
			Class.forName("com.mysql.cj.jdbc.Driver");
		} catch (ClassNotFoundException e) {
			App.logError("Unable to load MySQL Driver. Make sure you have it installed! Exiting");
			App.logDebug(ExceptionUtils.getStackTrace(e));
			System.exit(1);
		}
		
		App.logInfo("Connecting to the database...");
		
		try {
			connection = DriverManager.getConnection("jdbc:mysql://" + Config.mysqlHost + "/" + Config.mysqlDb + "?user=" + Config.mysqlUser + "&password=" +  Config.mysqlPassword);
		} catch (SQLException e) {
			App.logError("Unable to connect to the specified database! Exiting");
			App.logDebug(ExceptionUtils.getStackTrace(e));
			System.exit(1);
		}
		
		App.logInfo("Connection with database established.");
	}
	
	public ResultObject executeStatement(StatementType type, String statement) throws SQLException {
		PreparedStatement preparedStatement = connection.prepareStatement(statement);
		
		if(type == StatementType.query) {
			ResultSet resultSet = preparedStatement.executeQuery();
			return new ResultObject(type, resultSet);
		} else if(type == StatementType.update) {
			int resultInt = preparedStatement.executeUpdate();
			return new ResultObject(type, resultInt);
		}
		
		return null;
	}
}
