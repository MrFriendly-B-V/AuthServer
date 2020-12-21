package nl.thedutchmc.authserver;

import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.concurrent.ConcurrentHashMap;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

import nl.thedutchmc.authserver.database.SqlManager;
import nl.thedutchmc.authserver.runnables.DiscoveryDocument;

@SpringBootApplication
public class App {
	
	public static List<String> csrfTokens = new ArrayList<>();
	public static String authEndpoint = "";
	public static ConcurrentHashMap<String, User> userMap = new ConcurrentHashMap<>();
	public static HashMap<String, String> returnUri = new HashMap<String, String>();
	
	private static boolean DEBUG = true;
	private static SqlManager sqlManager;
	
	public static void main(String[] args) {
		App.logInfo("Welcome to AuthServer.");
		
		//Read config.json and storage.json
		Config config = new Config();
		config.readConfig();
		config.readStorage();
		
		//Set up the SqlManager
		sqlManager = new SqlManager();
		
		//Start the Spring boot server
		SpringApplication.run(App.class, args);
		
		//Get the Auth endpoint from Google
		if(App.authEndpoint.equals("")) {
			Thread discoveryDocumentThread = new Thread(new DiscoveryDocument());
			discoveryDocumentThread.start();	
		}
	}
	
	public static void logDebug(Object log) {
		if(!DEBUG) return;
		
		//kk:mm:ss --> hour:minute:seconds, without hours going 0-24
		final DateTimeFormatter f = DateTimeFormatter.ofPattern("kk:mm:ss");
		LocalDateTime now = LocalDateTime.now();
		System.out.println("[" + now.format(f) + "][DEBUG] " + log.toString());
	}
	
	public static void logInfo(Object log) {
		//kk:mm:ss --> hour:minute:seconds, without hours going 0-24
		final DateTimeFormatter f = DateTimeFormatter.ofPattern("kk:mm:ss");
		LocalDateTime now = LocalDateTime.now();
		System.out.println("[" + now.format(f) + "][INFO] " + log.toString());
	}
	
	public static void logError(Object log) {
		//kk:mm:ss --> hour:minute:seconds, without hours going 0-24
		final DateTimeFormatter f = DateTimeFormatter.ofPattern("kk:mm:ss");
		LocalDateTime now = LocalDateTime.now();
		System.err.println("[" + now.format(f) + "][ERROR] " + log.toString());
	}    
	
	public static SqlManager getSqlManager() {
		return sqlManager;
	}
}
