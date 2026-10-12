//! Ported from de.hysky.skyblocker.utils.LogsFolderFinder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::util::util;

/// java: private static @Nullable Path folder;
pub static FOLDER: compat::MainThread<Option<std::path::PathBuf>> = compat::MainThread::new(|| None);

/// java: public final class LogsFolderFinder
#[derive(Clone)]
pub struct LogsFolderFinder {}

impl LogsFolderFinder {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static Path getLogsFolder()
    pub fn get_logs_folder() -> std::path::PathBuf {
        if FOLDER.get_mut().is_none() {
            *FOLDER.get_mut() = Some(LogsFolderFinder::compute_folder());
        }
        return FOLDER.get_mut().clone().unwrap();
    }

    /// java: public static void openLogsFolder()
    pub fn open_logs_folder() {
        util::get_platform().open_path(&LogsFolderFinder::get_logs_folder());
    }

    /// java: @Init public static void init()
    pub fn init() {
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(..);
            },
        ));
    }

    /// java: private static Path computeFolder()
    pub fn compute_folder() -> std::path::PathBuf {
        // java: LoggerContext context = LogManager.getFactory().isClassLoaderDependent()
        //         ? LogManager.getContext(Minecraft.class.getClassLoader(), false) : LogManager.getContext(false);
        // java: if (!(context instanceof LoggerContext instance)) { LOGGER.warn(..); return getFallbackFolder(); }
        // rust: port gap - the game logs without log4j, so there is no `LoggerContext` (and no file appender) to ask
        //   (GAPS_batchE.md): the `instanceof` arm is the one taken
        let context: Option<crate::unresolved::LoggerContext> = None;
        let Some(instance) = context else {
            logger().warn("Could not compute logs folder, expected instance of LoggerContext got null");
            return LogsFolderFinder::get_fallback_folder();
        };
        let _ = instance;
        // TODO(fabric2seer): `.getConfiguration(..)` on a LoggerContext
        //   java: Map<String, Appender> appenders = instance.getConfiguration().getAppenders();
        let mut appenders: std::collections::HashMap<String, crate::unresolved::Appender> = todo!("fabric2seer: the initialiser of `appenders` (TODO above)");
        let mut file_names: Vec<String> = Vec::with_capacity(((appenders.len() as i32)) as usize);
        for appender in appenders.values() {
            // TODO(fabric2seer): `.getManager(..)` on a AbstractOutputStreamAppender
            //   java: if (appender instanceof AbstractOutputStreamAppender<?> streamAppender && streamAppender.getManager() instanceof FileManager manager) {
            //   java: 				fileNames.add(manager.getFileName());
            //   java: 			}
        }
        if file_names.is_empty() {
            logger().warn("Could not compute logs folder: didn't find any StreamAppender with a FileManager");
            return LogsFolderFinder::get_fallback_folder();
        }
        let mut latest_log_opt: Option<String> = file_names.iter().cloned().filter(|s| s.contains("latest.log")).next();
        if latest_log_opt.is_none() {
            logger().warn(&format!("Could not compute logs folder: didn't find a path that contains 'latest.log'. Only got: {}", format!("[{}]", file_names.join(", "))));
            return LogsFolderFinder::get_fallback_folder();
        }
        return (std::path::absolute(&std::path::PathBuf::from(&latest_log_opt.clone().unwrap())).unwrap_or_else(|_| std::path::PathBuf::from(&latest_log_opt.clone().unwrap()).clone())).parent().map(|x| x.to_path_buf()).unwrap();
    }

    /// java: private static Path getFallbackFolder()
    pub fn get_fallback_folder() -> std::path::PathBuf {
        return std::path::absolute(&mcrust::mcrust::modding::loader::game_dir().join("logs")).unwrap_or_else(|_| mcrust::mcrust::modding::loader::game_dir().join("logs").clone());
    }

}
