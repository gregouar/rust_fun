use crate::text_rendering::TextRenderer;

pub trait GameState {
    fn entering(&self);
    fn revealing(&self);
    fn obscuring(&self);
    fn leaving(&self);

    fn update(&self);
    fn draw(&self, text_renderer: &dyn TextRenderer);

    // fn handle_events(events_manager: &EventsManager);
    // fn update();
    // fn draw();

    // fn set_manager(states_manager: &StatesManager);

    // public:

    //     virtual void handleEvents(const EventsManager *eventsManager) = 0;
    //     virtual void update(const Time &elapsedTime) = 0;
    //     virtual void draw(RenderWindow *renderWindow) = 0;

    //     void setManager(StatesManager *);

    //     void pause();
    //     void resume();

    // protected:
    //     StatesManager *m_manager;

    // private:
    //     bool m_running;
}
