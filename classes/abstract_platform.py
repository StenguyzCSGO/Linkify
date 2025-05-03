from abc import ABC, abstractmethod

class Platform(ABC):
    def __init__(self, url):
        self.url = url

    @abstractmethod
    def get_title(self):
        pass

    @abstractmethod
    def get_artist(self):
        pass

    @abstractmethod
    def get_link(self):
        pass
    
    @abstractmethod
    def search_link(self, track_name, track_artist):
        pass